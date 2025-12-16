pub mod open_ai;

/// This function routes the incoming API requests to the appropriate handler based on the path.
pub async fn router(
    path: &str,
    structBody: serde_json::Value,
) -> Result<serde_json::Value, String> {
    match path {
        "/v1/chat/completions" => open_ai::ChatCompletions(structBody).await,
        _ => Err(format!("Unknown API path: {}", path)),
    }
}
