use backend::polytheus::Polytheus;
use dotenvy::dotenv;

use backend::hosting_method::aws_lambda;
use lambda_http::{run, service_fn, tracing, Error};

use base64::prelude::*;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    /*let image_path = "media/test/Sagrada_Familia.jpg";
    println!("image_path: {}", image_path);
    let mut file = std::fs::File::open(image_path).expect("Failed to open image file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read image file");
    let encoded_image = BASE64_STANDARD.encode(&buffer);
    let image_data_url = ImageType::Str(format!("data:image/jpeg;base64,{}", encoded_image));
    //let vec_image = ImageType::Vec(vec![image_data_url]);

    let polytheus = Polytheus::new();
    match polytheus
        .run_a_model(
            "GPT 5 codex",
            None,
            "What is this famous human construction ?",
            Some(&image_data_url),
            None,
            None,
            None,
        )
        .await
    {
        Ok(result) => println!("Model output: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }*/

    aws_lambda::run().await
}
