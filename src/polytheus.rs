use crate::ai_mnemosyne::ai_mnemosyne_impl::{
    AIMnemosyne, ImageParameterType, PredictionResponse, Provider,
};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::env;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ImageType {
    Str(String),
    Vec(Vec<String>),
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageType::Str(s) => write!(f, "{}", s),
            ImageType::Vec(v) => write!(f, "{:?}", v),
        }
    }
}

#[derive(Debug)]
pub struct Polytheus {
    pub ai_mnemosyne: AIMnemosyne,
}

impl Polytheus {
    pub fn new() -> Self {
        Self {
            ai_mnemosyne: AIMnemosyne::fill(),
        }
    }

    pub async fn run_a_model(
        &self,
        model_name: &str,
        thinking_level: Option<&str>,
        input_text: &str,
        input_image: Option<&ImageType /*String*/>,
        input_audio: Option<&str>,
        input_audio_format: Option<&str>,
        input_video: Option<&str>,
    ) -> Result<String, String> {
        println!("--- Preparing to run model '{}' ---", model_name);
        // Find the model by name
        let model = self
            .ai_mnemosyne
            .get_model_by_name(model_name)
            .ok_or_else(|| format!("Model '{}' not found", model_name))?;

        if let Some(image) = input_image {
            // Check image type compatibility
            if let Some(expected_type) = model.get_image_parameter_type() {
                match (expected_type, image) {
                    (ImageParameterType::String, ImageType::Vec(_)) => {
                        return Err(format!(
                                    "Model '{}' expects a single image string, but received a vector of images.",
                                    model_name
                                ));
                    }
                    (ImageParameterType::VecString, ImageType::Str(_)) => {
                        return Err(format!(
                                    "Model '{}' expects a vector of images, but received a single image string.",
                                    model_name
                                ));
                    }
                    _ => {} // Compatible
                }
            }
        }

        let thinking_level_property = model.get_thinking_level_property();

        // Check thinking level compatibility
        if let Some(_) = thinking_level {
            if !model
                .get_thinking_levels_authorized()
                .as_ref()
                .map(|levels| levels.contains(&thinking_level.unwrap_or_default().to_string()))
                .unwrap_or(false)
            {
                return Err(format!(
                    "thinking level '{}' is not authorized for model '{}'",
                    thinking_level.unwrap_or_default(),
                    model_name
                ));
            }
        }

        let client = Client::new();

        match model.get_provider() {
            // code for running the model if it's a replicate model
            Provider::Replicate => {
                println!("--- Using Replicate Provider ---");
                let api_token = env::var("REPLICATE_API_TOKEN")
                    .map_err(|_| "REPLICATE_API_TOKEN not set".to_string())?;

                println!("--- API Token Retrieved: {} ---", api_token);

                let url = &model.get_apiurl();

                println!("--- API URL Retrieved: {} ---", url);

                // Create a mutable JSON map
                let mut map = Map::new();

                // Insert dynamic key-value pair
                map.insert(
                    thinking_level_property.unwrap_or_default().to_string(),
                    json!(thinking_level.unwrap_or_default()),
                );

                // Convert map into JSON value
                let thinking = Value::Object(map);

                println!("{}", thinking);

                // Build the request body as a JSON object and conditionally include the thinking field.
                let mut body_map = Map::new();
                body_map.insert("stream".to_string(), json!(true));
                let mut input_map = Map::new();
                input_map.insert("prompt".to_string(), json!(input_text.to_string()));
                if let Some(image) = input_image {
                    let val = match image {
                        ImageType::Str(s) => json!(s),
                        ImageType::Vec(v) => json!(v),
                    };
                    println!("grosse pute {}", val);
                    input_map.insert(model.get_image_parameters().unwrap().to_string(), val);
                }

                if input_audio.is_some() {
                    input_map.insert("audio".to_string(), json!(input_audio.unwrap()));
                }
                if input_video.is_some() {
                    input_map.insert("video".to_string(), json!(input_video.unwrap()));
                }
                if thinking_level.is_some() && thinking_level_property.is_some() {
                    {
                        let prop = thinking_level_property.unwrap();
                        let tl = thinking_level.unwrap_or_default().trim();
                        // Try bool -> int -> float -> fallback to string
                        let val = if let Ok(b) = tl.parse::<bool>() {
                            Value::Bool(b)
                        } else if let Ok(i) = tl.parse::<i64>() {
                            Value::Number(i.into())
                        } else if let Ok(f) = tl.parse::<f64>() {
                            if let Some(n) = serde_json::Number::from_f64(f) {
                                Value::Number(n)
                            } else {
                                Value::String(tl.to_string())
                            }
                        } else {
                            Value::String(tl.to_string())
                        };
                        input_map.insert(prop.to_string(), val);
                    }
                }

                body_map.insert("input".to_string(), Value::Object(input_map));

                let body = Value::Object(body_map);

                println!("request body: {}", body);

                // 4. POST Request to get the stream_url
                let response = client
                    .post(*url)
                    .header("Authorization", format!("Bearer {}", &api_token))
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to send request: {}", e))?;

                println!("--- API Response Received ---");

                // Capture status before consuming the response body (text/json consume the Response)
                let status = response.status();
                if status != StatusCode::OK && status != StatusCode::CREATED {
                    let error_text: String = response
                        .text()
                        .await
                        .map_err(|e| format!("error reading error body: {}", e))?;
                    return Err(format!(
                        "First API call failed with status: {} and body: {}",
                        status, error_text
                    ));
                }

                // 5. Deserialize the response to get the stream URL
                let prediction: PredictionResponse = response
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse prediction response: {}", e))?;
                let stream_url = prediction.urls.stream;

                // The equivalent of 'jq -r .urls.stream' is done by the serde deserialization above
                println!("--- Stream URL Retrieved: {} ---", stream_url);

                // 6. GET Request to the stream URL (Server-Sent Events)
                let mut stream_response = client
                    .get(&stream_url)
                    .header("Accept", "text/event-stream")
                    .header("Cache-Control", "no-store")
                    .header("Authorization", format!("Bearer {}", api_token)) // Optional, but good practice
                    .send()
                    .await
                    .map_err(|e| format!("Failed to send stream request: {}", e))?;

                println!("--- Stream Response Received ---");

                let stream_status = stream_response.status();
                if stream_status != StatusCode::OK {
                    let error_text = stream_response
                        .text()
                        .await
                        .map_err(|e| format!("error reading stream error body: {}", e))?;
                    return Err(format!(
                        "Stream API call failed with status: {} and body: {}",
                        stream_status, error_text
                    ));
                }

                // 7. Process the SSE stream chunk by chunk; parse events and stop on "done"
                // Accumulate partial chunks into `buffer` because SSE messages can be split across chunks.
                let mut buffer = String::new();
                loop {
                    let opt_chunk = stream_response
                        .chunk()
                        .await
                        .map_err(|e| format!("Stream chunk error: {}", e))?;

                    let chunk = match opt_chunk {
                        Some(c) => c,
                        None => break, // connection closed
                    };

                    // Append received bytes (lossy) to the buffer
                    buffer.push_str(&String::from_utf8_lossy(&chunk));

                    // Try to extract full events separated by empty line (handle both CRLF and LF)
                    loop {
                        // find earliest double-newline separator (CRLF or LF)
                        let sep_pos = buffer.find("\r\n\r\n").or_else(|| buffer.find("\n\n"));

                        if sep_pos.is_none() {
                            break; // no full event yet
                        }
                        let idx = sep_pos.unwrap();
                        let event_block = buffer[..idx].to_string();
                        // remove processed block + separator from buffer
                        // handle either CRLF or LF length
                        if buffer.get(idx..idx + 4) == Some("\r\n\r\n") {
                            buffer = buffer[idx + 4..].to_string();
                        } else {
                            buffer = buffer[idx + 2..].to_string();
                        }

                        // Parse the event block
                        let mut event_type: Option<String> = None;
                        let mut data_lines: Vec<String> = Vec::new();
                        for line in event_block.lines() {
                            if let Some(rest) = line.strip_prefix("event:") {
                                event_type = Some(rest.trim().to_string());
                            } else if let Some(rest) = line.strip_prefix("data:") {
                                data_lines.push(rest.trim().to_string());
                            }
                        }

                        let data = data_lines.join("\n");

                        // If the provider signals done, finish the stream handling immediately.
                        if event_type.as_deref() == Some("done") {
                            // Optionally print any final data (often `{}`) before returning.
                            if !data.is_empty() && data != "{}" {
                                print!("{}", data);
                                std::io::stdout()
                                    .flush()
                                    .map_err(|e| format!("Stdout flush error: {}", e))?;
                            }
                            println!("\n--- Stream 'done' event received ---");
                            return Ok(format!("Stream finished for model '{}'", model_name));
                        }

                        // For other events (or absent event field), print data if present.
                        if !data.is_empty() && data != "{}" {
                            print!("{}", data);
                            std::io::stdout()
                                .flush()
                                .map_err(|e| format!("Stdout flush error: {}", e))?;
                        }
                    }
                }
                // If we exit the read loop without receiving a "done" event, treat it as finished.
                println!("\n--- Stream Finished (connection closed) ---");
                return Ok(format!("Stream finished for model '{}'", model_name));
            }
            // code for running the model if it's an openrouter model
            Provider::OpenRouter => {
                println!("--- Using OpenRouter Provider ---");

                let api_key = env::var("OPENROUTER_API_KEY")
                    .map_err(|_| "OPENROUTER_API_KEY not set".to_string())?;
                println!("--- OPENROUTER_API_KEY retrieved ---");

                let url = "https://openrouter.ai/api/v1/chat/completions";

                // Choose which model id to send. Many people store a model id like "openai/gpt-5-codex"
                // in model.apiurl or model.name; adapt this as needed:
                let model_id = model.get_apiurl();

                // Build the messages payload: here we send a single user message containing a text content.
                // If you want to send images, you can push another content item with type = "image_url".
                let mut content_list =
                    vec![serde_json::json!({ "type": "text", "text": input_text })];

                if let Some(img) = input_image {
                    content_list.push(serde_json::json!({
                        "type": "image_url",
                        "image_url": {
                            "url": match img {
                                ImageType::Str(s) => json!(s),
                                ImageType::Vec(v) => json!(v),
                            }
                        }
                    }));
                }

                if let Some(aud) = input_audio {
                    content_list.push(serde_json::json!({
                        "type": "input_audio",
                        "inputAudio": {
                            "data": aud,
                            "format": input_audio_format.unwrap_or("unknown")
                        }
                    }));
                }

                if let Some(vid) = input_video {
                    content_list.push(serde_json::json!({
                        "type": "video_url",
                        "inputVideo": {
                            "url": vid
                        }
                    }));
                }

                let messages = vec![serde_json::json!({
                    "role": "user",
                    "content": content_list
                })];

                // Create a mutable JSON map
                let mut map = Map::new();

                // Insert dynamic key-value pair
                map.insert(
                    thinking_level_property.unwrap_or_default().to_string(),
                    json!(thinking_level.unwrap_or_default()),
                );

                // Convert map into JSON value
                let thinking = Value::Object(map);

                println!("{}", thinking);

                // Build the request body as a JSON object and conditionally include the thinking field.
                let mut body_map = Map::new();
                body_map.insert("model".to_string(), json!(model_id));
                body_map.insert("messages".to_string(), json!(messages));
                if thinking_level.is_some() && thinking_level_property.is_some() {
                    {
                        let prop = thinking_level_property.unwrap();
                        let tl = thinking_level.unwrap_or_default().trim();
                        // Try bool -> int -> float -> fallback to string
                        let val = if let Ok(b) = tl.parse::<bool>() {
                            Value::Bool(b)
                        } else if let Ok(i) = tl.parse::<i64>() {
                            Value::Number(i.into())
                        } else if let Ok(f) = tl.parse::<f64>() {
                            if let Some(n) = serde_json::Number::from_f64(f) {
                                Value::Number(n)
                            } else {
                                Value::String(tl.to_string())
                            }
                        } else {
                            Value::String(tl.to_string())
                        };
                        body_map.insert(prop.to_string(), val);
                    }
                }
                let body = Value::Object(body_map);

                println!("OpenRouter request body: {}", body);

                let response = client
                    .post(url)
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Content-Type", "application/json")
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| format!("Failed to send OpenRouter request: {}", e))?;

                println!("--- OpenRouter response received ---");

                let status = response.status();
                if !status.is_success() {
                    let error_text = response
                        .text()
                        .await
                        .map_err(|e| format!("error reading error body: {}", e))?;
                    return Err(format!(
                        "OpenRouter API call failed with status: {} and body: {}",
                        status, error_text
                    ));
                }

                // Parse the JSON response. Be permissive: OpenRouter's "content" may be an array of items.
                let resp_json: serde_json::Value = response
                    .json()
                    .await
                    .map_err(|e| format!("Failed to parse OpenRouter response JSON: {}", e))?;

                // Aggregate readable text from the common response shapes:
                let mut result_text = String::new();

                if let Some(choices) = resp_json.get("choices").and_then(|c| c.as_array()) {
                    for choice in choices {
                        // many OpenAI-like APIs put the message under choice["message"]["content"]
                        if let Some(message) = choice.get("message") {
                            if let Some(content) = message.get("content") {
                                if content.is_array() {
                                    for item in content.as_array().unwrap() {
                                        if let Some(typ) = item.get("type").and_then(|t| t.as_str())
                                        {
                                            match typ {
                                                "text" => {
                                                    if let Some(text) =
                                                        item.get("text").and_then(|t| t.as_str())
                                                    {
                                                        result_text.push_str(text);
                                                    }
                                                }
                                                "image_url" => {
                                                    if let Some(url) = item
                                                        .get("image_url")
                                                        .and_then(|iu| iu.get("url"))
                                                        .and_then(|u| u.as_str())
                                                    {
                                                        result_text.push_str(&format!(
                                                            "\n[image: {}]",
                                                            url
                                                        ));
                                                    }
                                                }
                                                other => {
                                                    // unknown content types: try to stringify a best-effort
                                                    if let Some(s) =
                                                        item.get("text").and_then(|t| t.as_str())
                                                    {
                                                        result_text.push_str(s);
                                                    } else {
                                                        result_text.push_str(&format!(
                                                            "\n[{} item]",
                                                            other
                                                        ));
                                                    }
                                                }
                                            }
                                        } else if let Some(s) = item.as_str() {
                                            // fallback: content item itself is a string
                                            result_text.push_str(s);
                                        }
                                    }
                                } else if let Some(s) = content.as_str() {
                                    // content is a plain string
                                    result_text.push_str(s);
                                } else {
                                    // last-resort: pretty-print content
                                    result_text.push_str(&format!("\n{}", content));
                                }
                            }
                        } else if let Some(txt) = choice.get("text").and_then(|t| t.as_str()) {
                            // Some APIs return a simple "text" field on choice
                            result_text.push_str(txt);
                        }
                    }
                } else {
                    // If there are no choices, try to pretty-print the whole response (for debugging)
                    result_text = serde_json::to_string_pretty(&resp_json)
                        .unwrap_or_else(|_| "OpenRouter: unknown response shape".to_string());
                }

                println!("--- OpenRouter aggregated result: {} ---", result_text);
                return Ok(result_text);
            }
        }
    }
}
