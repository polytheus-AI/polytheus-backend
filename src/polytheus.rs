use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::env;
use tokio::time::{sleep, Duration};

mod model;
use model::{Model, Provider};

mod organization;
use organization::Organization;

mod licence;
use licence::Licence;

mod benchmark;
use benchmark::Benchmark;

/// Build a Replicate prediction request body.
///
/// This is intentionally "model-agnostic" and sends OpenAI-like `messages` under `input.messages`,
/// plus an optional thinking-level property if the model declares one.
fn build_replicate_request_body(
    messages: &[Message],
    thinking_level_property: Option<&str>,
    thinking_level: Option<&str>,
    stream: bool,
) -> Result<Value, String> {
    let mut body_map = Map::new();
    body_map.insert("stream".to_string(), json!(stream));

    let mut input_map = Map::new();

    // Convert messages to OpenAI-like format
    let formatted_messages: Vec<Value> = messages
        .iter()
        .map(|msg| {
            let mut content_list = vec![json!({ "type": "text", "text": msg.input_text })];

            if let Some(img) = &msg.input_image {
                content_list.push(json!({
                    "type": "image_url",
                    "image_url": {
                        "url": json!(img)
                    }
                }));
            }
            if let Some(aud) = &msg.input_audio {
                content_list.push(json!({
                    "type": "input_audio",
                    "inputAudio": {
                        "data": aud,
                        "format": msg.input_audio_format.as_deref().unwrap_or("unknown")
                    }
                }));
            }
            if let Some(vid) = &msg.input_video {
                content_list.push(json!({
                    "type": "video_url",
                    "inputVideo": {
                        "url": vid
                    }
                }));
            }

            json!({
                "role": msg.role,
                "content": content_list
            })
        })
        .collect();

    input_map.insert("messages".to_string(), json!(formatted_messages));

    // Handle thinking level
    if let (Some(tl_prop), Some(tl)) = (thinking_level_property, thinking_level) {
        let tl_val = tl.trim();
        let val = if let Ok(b) = tl_val.parse::<bool>() {
            Value::Bool(b)
        } else if let Ok(i) = tl_val.parse::<i64>() {
            Value::Number(i.into())
        } else if let Ok(f) = tl_val.parse::<f64>() {
            if let Some(n) = serde_json::Number::from_f64(f) {
                Value::Number(n)
            } else {
                Value::String(tl_val.to_string())
            }
        } else {
            Value::String(tl_val.to_string())
        };
        input_map.insert(tl_prop.to_string(), val);
    }

    body_map.insert("input".to_string(), Value::Object(input_map));
    Ok(Value::Object(body_map))
}

/// Extract best-effort readable text from a Replicate `output` JSON value.
fn extract_replicate_output_text(output: &Value) -> String {
    match output {
        Value::String(s) => s.clone(),
        Value::Array(items) => {
            let mut out = String::new();
            for item in items {
                match item {
                    Value::String(s) => out.push_str(s),
                    other => out.push_str(&other.to_string()),
                }
            }
            out
        }
        other => other.to_string(),
    }
}

#[derive(Deserialize, Debug)]
pub struct PredictionUrls {
    /// URL used for Server-Sent Events streaming (optional).
    pub stream: Option<String>,

    /// URL used to fetch the prediction status/result (optional).
    pub get: Option<String>,

    /// URL used to cancel the prediction (optional).
    pub cancel: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PredictionResponse {
    /// Prediction identifier.
    pub id: Option<String>,

    /// Prediction status (e.g. "starting", "processing", "succeeded", "failed").
    pub status: Option<String>,

    /// Prediction output; shape is model-dependent.
    pub output: Option<Value>,

    /// URLs related to this prediction.
    pub urls: Option<PredictionUrls>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub input_text: String,
    pub input_image: Option<String>,
    pub input_audio: Option<String>,
    pub input_audio_format: Option<String>,
    pub input_video: Option<String>,
}

#[derive(Debug)]
pub struct Polytheus {
    models: Vec<Model>,
    organizations: Option<Vec<Organization>>,

    licences: Option<Vec<Licence>>,
    benchmarks: Vec<Benchmark>,
}

impl Polytheus {
    /// fill Polytheus with all the object obligated to allow Polytheus to work
    /// that says models and benchmarks
    pub fn fast_fill() -> Polytheus {
        Polytheus {
            models: Model::fill(),
            organizations: None,
            licences: None,
            benchmarks: Benchmark::fill(),
        }
    }

    pub async fn run(
        &self,
        model_name: &str,
        messages: Vec<Message>,
        thinking_level: Option<String>,
    ) -> Result<String, String> {
        println!("--- Preparing to run model '{}' ---", model_name);
        // Find the model by name
        let model = self
            .get_model_by_name(model_name)
            .ok_or_else(|| format!("Model '{}' not found", model_name))?;

        let client = Client::new();

        // Extract thinking level from the last message if present
        let thinking_level_property = model.get_thinking_level_property();

        let thinking_levels_authorized = model.get_thinking_levels_authorized();
        if let Some(tla) = &thinking_levels_authorized {
            if let Some(tl) = &thinking_level {
                if !tla.contains(&tl) {
                    return Err(format!(
                        "Thinking level '{}' not authorized for model '{}'",
                        tl, model_name
                    ));
                }
            }
        }

        let roles_authorized = model.get_roles_authorized();
        if let Some(ra) = &roles_authorized {
            for msg in &messages {
                if !ra.contains(&msg.role) {
                    return Err(format!(
                        "Role '{}' not authorized for model '{}'",
                        msg.role, model_name
                    ));
                }
            }
        }

        match model.get_provider() {
            // code for running the model if it's a replicate model
            Provider::Replicate => {
                println!("--- Using Replicate Provider ---");
                let api_token = env::var("REPLICATE_API_TOKEN")
                    .map_err(|_| "REPLICATE_API_TOKEN not set".to_string())?;

                println!("--- REPLICATE_API_TOKEN retrieved ---");

                let url = &model.get_apiurl();

                println!("--- API URL Retrieved: {} ---", url);

                // Default behavior: non-streaming (poll the prediction "get" URL and return a normal response).
                let body = build_replicate_request_body(
                    &messages,
                    thinking_level_property,
                    thinking_level.as_deref(),
                    false,
                )?;

                println!("request body: {}", body);

                // 4. POST Request to get the stream_url
                let response = client
                    .post(*url)
                    .header("Authorization", format!("Bearer {}", api_token))
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

                // If Replicate already returned an output (rare for async predictions), return it.
                if let Some(output) = prediction.output.as_ref() {
                    return Ok(extract_replicate_output_text(output));
                }

                // Poll the prediction "get" URL until it succeeds.
                let get_url = prediction
                    .urls
                    .as_ref()
                    .and_then(|u| u.get.as_deref())
                    .map(|s| s.to_string())
                    .or_else(|| {
                        prediction
                            .id
                            .as_deref()
                            .map(|id| format!("https://api.replicate.com/v1/predictions/{}", id))
                    })
                    .ok_or_else(|| {
                        "Replicate prediction response missing urls.get and id".to_string()
                    })?;

                let timeout = Duration::from_secs(120);
                let mut delay = Duration::from_millis(200);
                let start = std::time::Instant::now();

                loop {
                    if start.elapsed() > timeout {
                        return Err(format!(
                            "Replicate prediction timed out after {:?}",
                            timeout
                        ));
                    }

                    let poll_resp = client
                        .get(&get_url)
                        .header("Authorization", format!("Bearer {}", api_token))
                        .send()
                        .await
                        .map_err(|e| format!("Failed to poll prediction: {}", e))?;

                    let poll_status = poll_resp.status();
                    if !poll_status.is_success() {
                        let error_text = poll_resp
                            .text()
                            .await
                            .map_err(|e| format!("error reading poll error body: {}", e))?;
                        return Err(format!(
                            "Replicate poll failed with status: {} and body: {}",
                            poll_status, error_text
                        ));
                    }

                    let poll_json: Value = poll_resp
                        .json()
                        .await
                        .map_err(|e| format!("Failed to parse poll response JSON: {}", e))?;

                    let status_str = poll_json
                        .get("status")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");

                    match status_str {
                        "succeeded" => {
                            let output = poll_json.get("output").ok_or_else(|| {
                                format!("Replicate succeeded but missing output: {}", poll_json)
                            })?;
                            return Ok(extract_replicate_output_text(output));
                        }
                        "failed" | "canceled" => {
                            return Err(format!(
                                "Replicate prediction {}: {}",
                                status_str, poll_json
                            ));
                        }
                        _ => {
                            sleep(delay).await;
                            delay = std::cmp::min(delay * 2, Duration::from_secs(2));
                        }
                    }
                }
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

                // Build the messages payload
                let formatted_messages: Vec<Value> = messages
                    .iter()
                    .map(|msg| {
                        let mut content_list =
                            vec![json!({ "type": "text", "text": msg.input_text })];

                        if let Some(img) = &msg.input_image {
                            content_list.push(json!({
                                "type": "image_url",
                                "image_url": {
                                    "url": img
                                }
                            }));
                        }
                        if let Some(aud) = &msg.input_audio {
                            content_list.push(json!({
                                "type": "input_audio",
                                "inputAudio": {
                                    "data": aud,
                                    "format": msg.input_audio_format.as_deref().unwrap_or("unknown")
                                }
                            }));
                        }
                        if let Some(vid) = &msg.input_video {
                            content_list.push(json!({
                                "type": "video_url",
                                "inputVideo": {
                                    "url": vid
                                }
                            }));
                        }

                        Ok(json!({
                            "role": msg.role,
                            "content": content_list
                        }))
                    })
                    .collect::<Result<Vec<Value>, String>>()?;

                println!("OpenRouter formatted messages: {:?}", formatted_messages);

                // Build the request body
                let mut body_map = Map::new();
                body_map.insert("model".to_string(), json!(model_id));
                body_map.insert("messages".to_string(), json!(formatted_messages));

                // Handle thinking level
                if let Some(tl_prop) = thinking_level_property {
                    if let Some(tl) = &thinking_level {
                        let tl_val = tl.trim();
                        let val = if let Ok(b) = tl_val.parse::<bool>() {
                            Value::Bool(b)
                        } else if let Ok(i) = tl_val.parse::<i64>() {
                            Value::Number(i.into())
                        } else if let Ok(f) = tl_val.parse::<f64>() {
                            if let Some(n) = serde_json::Number::from_f64(f) {
                                Value::Number(n)
                            } else {
                                Value::String(tl_val.to_string())
                            }
                        } else {
                            Value::String(tl_val.to_string())
                        };
                        body_map.insert(tl_prop.to_string(), val);
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

    /// getter for the model by its name
    pub fn get_model_by_name(&self, model_name: &str) -> Option<&Model> {
        self.models
            .iter()
            .find(|model| model.get_name() == model_name)
    }

    /// getter for the benchmark by its name
    pub fn get_benchmark_by_name(&self, benchmark_name: &str) -> Option<&Benchmark> {
        self.benchmarks
            .iter()
            .find(|benchmark| benchmark.get_name() == benchmark_name)
    }
}

#[cfg(test)]
mod replicate_non_stream_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_build_replicate_request_body_stream_flag_false() {
        let start = Instant::now();

        let messages = vec![Message {
            role: "user".to_string(),
            input_text: "hello".to_string(),
            input_image: None,
            input_audio: None,
            input_audio_format: None,
            input_video: None,
        }];

        let body = build_replicate_request_body(&messages, None, None, false).unwrap();
        assert_eq!(body.get("stream").and_then(|v| v.as_bool()), Some(false));

        let duration = Instant::now() - start;
        eprintln!(
            "test_build_replicate_request_body_stream_flag_false took: {:?}",
            duration
        );
    }

    #[test]
    fn test_extract_replicate_output_text_string_and_array() {
        let start = Instant::now();

        let s = Value::String("abc".to_string());
        assert_eq!(extract_replicate_output_text(&s), "abc".to_string());

        let a = Value::Array(vec![
            Value::String("a".to_string()),
            Value::String("b".to_string()),
        ]);
        assert_eq!(extract_replicate_output_text(&a), "ab".to_string());

        let duration = Instant::now() - start;
        eprintln!(
            "test_extract_replicate_output_text_string_and_array took: {:?}",
            duration
        );
    }
}
