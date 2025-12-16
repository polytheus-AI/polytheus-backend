use backend::polytheus::{Message, Polytheus};
use serde_json::json;

use base64::prelude::*;
use std::io::Read;

async fn test(model_name: String) {
    dotenvy::dotenv().ok();
    let poly = Polytheus::fast_fill();

    // Test with Replicate gpt-4o-mini (assuming it is configured in AIMnemosyne)
    let messages = vec![Message {
        role: "user".to_string(),
        input_text: "Hello, this is a test message for Replicate.".to_string(),
        input_image: None,
        input_audio: None,
        input_audio_format: None,
        input_video: None,
    }];

    println!("Testing Replicate gpt-4o-mini...");
    match poly.run(&model_name, messages, None).await {
        Ok(res) => println!("Replicate Success: {}", res),
        Err(e) => eprintln!("Replicate Error: {}", e),
    }
}

async fn test_with_image(model_name: String) {
    dotenvy::dotenv().ok();

    let image_path = "media/test/Sagrada_Familia.jpg";
    println!("image_path: {}", image_path);
    let mut file = std::fs::File::open(image_path).expect("Failed to open image file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read image file");
    let encoded_image = BASE64_STANDARD.encode(&buffer);
    let image_data_url = format!("data:image/jpeg;base64,{}", encoded_image);
    //let vec_image = ImageType::Vec(vec![image_data_url]);

    let poly = Polytheus::fast_fill();

    let messages = vec![Message {
        role: "user".to_string(),
        input_text: "What is in this image?".to_string(),
        input_image: Some(image_data_url),
        input_audio: None,
        input_audio_format: None,
        input_video: None,
    }];

    println!("Testing Image Input...");
    // Use a vision capable model
    match poly.run(&model_name, messages, None).await {
        Ok(res) => println!("Image Test Success: {}", res),
        Err(e) => eprintln!("Image Test Error: {}", e),
    }
}

async fn test_with_multiple_messages(model_name: String) {
    dotenvy::dotenv().ok();

    let poly = Polytheus::fast_fill();

    let messages = vec![
        Message {
            role: "user".to_string(),
            input_text: "How are you?".to_string(),
            input_image: None,
            input_audio: None,
            input_audio_format: None,
            input_video: None,
        },
        Message {
            role: "user".to_string(),
            input_text: "What is the meaning of life?".to_string(),
            input_image: None,
            input_audio: None,
            input_audio_format: None,
            input_video: None,
        },
    ];

    println!("Testing Image Input...");
    // Use a vision capable model
    match poly.run(&model_name, messages, None).await {
        Ok(res) => println!("Image Test Success: {}", res),
        Err(e) => eprintln!("Image Test Error: {}", e),
    }
}

#[tokio::test]
async fn test_run_llm_replicate() {
    test("gpt-4o-mini".to_string()).await;
}

#[tokio::test]
async fn test_run_llm_openrouter() {
    test("grok-4-fast".to_string()).await;
}

#[tokio::test]
async fn test_run_llm_with_image_replicate() {
    test_with_image("gpt-4o-mini".to_string()).await;
}

#[tokio::test]
async fn test_run_llm_with_image_openrouter() {
    test_with_image("grok-4-fast".to_string()).await;
}

#[tokio::test]
async fn test_run_llm_with_multiple_messages_replicate() {
    test_with_multiple_messages("gpt-4o-mini".to_string()).await;
}

#[tokio::test]
async fn test_run_llm_with_multiple_messages_openrouter() {
    test_with_multiple_messages("grok-4-fast".to_string()).await;
}
