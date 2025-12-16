/// This file simulates an OpenAI-compatible API using the Polytheus backend.
/// It translates OpenAI API requests into Polytheus calls and formats the responses accordingly.
use crate::polytheus::{Message, Polytheus};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

/// Handles Open AI API that use chat completions endpoint.
pub async fn ChatCompletions(structBody: serde_json::Value) -> Result<serde_json::Value, String> {
    let polytheus = Polytheus::fast_fill();
    let model_name = structBody["model"]
        .as_str()
        .ok_or("you are missing the model name".to_string())?;
    let messages_json = structBody["messages"]
        .as_array()
        .ok_or("you are missing the messages".to_string())?;
    let reasoning_effort = structBody["reasoning_effort"]
        .as_str()
        .map(|s| s.to_string());
    let mut messages: Vec<Message> = vec![];
    for message_json in messages_json {
        let role = message_json["role"]
            .as_str()
            .ok_or("you are missing the role")?
            .to_string();
        let input_text = if message_json.get("content").is_none() {
            return Err("you are missing the content".to_string());
        } else if let Some(s) = message_json["content"].as_str() {
            s.to_string()
        } else if let Some(arr) = message_json["content"].as_array() {
            let mut parts: Vec<String> = Vec::new();
            for el in arr {
                // element might be plain string
                if let Some(t) = el.as_str() {
                    parts.push(t.to_string());
                    continue;
                }
                // element might be an object like { "type": "text", "text": "..." }
                if el.is_object() {
                    if let Some(t) = el.get("text").and_then(|v| v.as_str()) {
                        parts.push(t.to_string());
                        continue;
                    }
                    // some variants use "content" or nested structures:
                    if let Some(t) = el.get("content").and_then(|v| v.as_str()) {
                        parts.push(t.to_string());
                        continue;
                    }
                    // handle arrays-of-arrays if necessary (flatten one level)
                    if let Some(inner_arr) = el.get("content").and_then(|v| v.as_array()) {
                        for inner in inner_arr {
                            if let Some(t) = inner.get("text").and_then(|v| v.as_str()) {
                                parts.push(t.to_string());
                            } else if let Some(t) = inner.as_str() {
                                parts.push(t.to_string());
                            }
                        }
                    }
                }
            }
            if parts.is_empty() {
                return Err("you are missing the content".to_string());
            }
            parts.join(" ")
        } else {
            return Err("you are missing the content".to_string());
        };
        let input_image = message_json["input_image"].as_str().map(|s| s.to_string());
        let input_audio = message_json["input_audio"].as_str().map(|s| s.to_string());
        let input_audio_format = message_json["input_audio_format"]
            .as_str()
            .map(|s| s.to_string());
        let input_video = message_json["input_video"].as_str().map(|s| s.to_string());

        messages.push(Message {
            role,
            input_text,
            input_image,
            input_audio,
            input_audio_format,
            input_video,
        });
    }

    // Handle response_format option (Structured Outputs / JSON modes)
    // Note: the OpenAI Python SDK typically sends JSON Schema nested like:
    // { "type": "json_schema", "json_schema": { "name": "...", "schema": { ... }, "strict": true } }
    let mut response_format_type: Option<String> = None;
    if let Some(rf) = structBody.get("response_format") {
        if rf.is_object() {
            response_format_type = rf
                .get("type")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            let mut format_name: Option<String> = None;
            let mut format_description: Option<String> = None;
            let mut format_schema: Option<serde_json::Value> = None;
            let mut format_strict: bool = false;

            if response_format_type.as_deref() == Some("json_schema") {
                if let Some(js) = rf.get("json_schema") {
                    format_name = js
                        .get("name")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    format_description = js
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    format_schema = js.get("schema").cloned();
                    format_strict = js.get("strict").and_then(|v| v.as_bool()).unwrap_or(false);
                }
            }

            // If user provided a schema or json mode, insert a system instruction so the model knows
            if response_format_type.as_deref() == Some("json_schema")
                || response_format_type.as_deref() == Some("json_object")
            {
                let mut instr = String::new();
                if response_format_type.as_deref() == Some("json_schema") {
                    instr.push_str("You must respond with a single JSON object that conforms to the provided JSON Schema.");
                } else {
                    instr.push_str(
                        "You must respond with a single valid JSON object (no surrounding text).",
                    );
                }

                if let Some(name) = &format_name {
                    instr.push_str(&format!(" Name: {}.", name));
                }
                if let Some(desc) = &format_description {
                    instr.push_str(&format!(" Description: {}.", desc));
                }
                if let Some(schema) = &format_schema {
                    instr.push_str("\n\nJSON Schema:\n");
                    instr.push_str(&schema.to_string());
                }
                if format_strict {
                    instr.push_str("\n\nStrict mode: follow the schema exactly and do not include any extra fields or surrounding explanatory text.");
                } else {
                    instr.push_str("\n\nIf you cannot fully satisfy the schema, return the best-effort JSON object.");
                }

                // Prepend the system instruction so it's the first message the model sees
                messages.insert(
                    0,
                    Message {
                        role: "system".to_string(),
                        input_text: instr,
                        input_image: None,
                        input_audio: None,
                        input_audio_format: None,
                        input_video: None,
                    },
                );
            }
        }
    }

    println!("model_name: {}", model_name);
    let result_text = polytheus
        .run(model_name, messages.clone(), reasoning_effort)
        .await
        .map_err(|e| {
            "Something go wrong with the Polytheus run. Polytheus error: ".to_string()
                + &e.to_string()
        })?;

    // Build OpenAI-like response
    let created = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("time error: {}", e))?
        .as_secs();

    // simple id using timestamp (replace with stronger id if desired)
    let id = format!("chatcmpl-{}", created);

    // compute naive token counts by word counts as a lightweight approximation
    let prompt_tokens: usize = messages
        .iter()
        .map(|m| m.input_text.split_whitespace().count())
        .sum();
    let completion_tokens: usize = result_text.split_whitespace().count();
    let total_tokens = prompt_tokens + completion_tokens;

    let choice = json!({
        "index": 0,
        "message": {
            "role": "assistant",
            // IMPORTANT: keep `content` as a string for OpenAI SDK compatibility.
            // The OpenAI Python `.parse()` helper expects `message.content` to be a JSON *string*
            // (the SDK parses/validates it client-side into `.message.parsed`).
            "content": result_text,
            "refusal": Value::Null,
            "annotations": []
        },
        "logprobs": Value::Null,
        "finish_reason": "stop"
    });

    let usage = json!({
        "prompt_tokens": prompt_tokens,
        "completion_tokens": completion_tokens,
        "total_tokens": total_tokens,
        "prompt_tokens_details": {
            "cached_tokens": 0,
            "audio_tokens": 0
        },
        "completion_tokens_details": {
            "reasoning_tokens": 0,
            "audio_tokens": 0,
            "accepted_prediction_tokens": 0,
            "rejected_prediction_tokens": 0
        }
    });

    let response = json!({
        "id": id,
        "object": "chat.completion",
        "created": created,
        "model": model_name,
        "choices": [ choice ],
        "usage": usage,
        "service_tier": "default"
    });

    Ok(response)
}
