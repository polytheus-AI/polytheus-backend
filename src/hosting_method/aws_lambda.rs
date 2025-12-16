use std::any::Any;
use std::fmt;
use std::ops::DerefMut;
use std::{borrow, result};

use lambda_http::aws_lambda_events::query_map::QueryMap;
use lambda_http::http::header;
use lambda_http::{service_fn, tracing, Body, Error, Request, RequestExt, Response};

use crate::polytheus::Message;

use serde_json::{ser, Value};

use crate::api;
use crate::polytheus::Polytheus;

pub async fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::init_default_subscriber();

    lambda_http::run(service_fn(function_handler)).await
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let path = event.uri().path(); // ex: "/users/42/posts/7"

    let body = event.body();

    println!("{:?}", body);

    let struct_body = serde_json::from_slice::<Value>(body.as_ref()).map_err(|e| {
        "Error to parsing the body of the request. deserialize error: ".to_string() + &e.to_string()
    })?;

    let result = api::router(path, struct_body).await;

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = match result {
        Ok(data) => Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&data)?))?,
        Err(e) => Response::builder()
            .status(400)
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&serde_json::json!({
                "error": e.to_string()
            }))?))?,
    };
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::{Request, RequestExt};
    use std::collections::HashMap;

    /*#[tokio::test]
    async fn test_http_handler_with_query_string() {
        let mut query_string_parameters: HashMap<String, String> = HashMap::new();
        query_string_parameters.insert("name".into(), "aws-lambda".into());

        let request = Request::default().with_query_string_parameters(query_string_parameters);

        let response = function_handler(request).await.unwrap();
        assert_eq!(response.status(), 200);

        let body_bytes = response.body().to_vec();
        let body_string = String::from_utf8(body_bytes).unwrap();

        assert_eq!(
            body_string,
            "Hello aws-lambda, this is an AWS Lambda HTTP request"
        );
    }*/
}
