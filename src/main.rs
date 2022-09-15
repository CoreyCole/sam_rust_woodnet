use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::{json, Value};
use serde::Serialize;
use serde::Deserialize;

#[derive(Deserialize)]
struct CustomRequest {
    username: String,
    password_hash: String,
}

#[derive(Serialize)]
struct CustomResponse {
    username: String,
    req: String,
}
/// This is the main body for the function.
/// Write your code inside it.
/// There are some code examples in the Runtime repository:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request
    match event.body() {
        Body::Text(payload) => {
            let req = serde_json::from_str::<CustomRequest>(payload)?;
            // Return something that implements IntoResponse.
            // It will be serialized to the right response event automatically by the runtime
            let response = CustomResponse {
                username: req.username,
                req: payload.clone(),
            };
            log::info!("{}", req.password_hash);
            let resp = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(json!(response).to_string().into())
                .map_err(Box::new)?;
            Ok(resp)
        },
        Body::Empty => {
            let resp = Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body("request body empty!".into())
                .map_err(Box::new)?;
            Ok(resp)
        },
        Body::Binary(payload) => {
            let req = serde_json::from_slice::<CustomRequest>(payload)?;
            let resp = Response::builder()
                .status(200)
                .header("Content-Type", "application/json")
                .body("request body binary!".into())
                .map_err(Box::new)?;
            Ok(resp)
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

/* use lambda_runtime::{service_fn, Context, LambdaEvent, Error as LambdaError};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;
use log;

#[derive(Deserialize)]
struct Request {
    username: String,
    password_hash: String
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(woodnet);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn woodnet(event: LambdaEvent<Request>) -> Result<Response, LambdaError> {
    // extract some useful info from the request
    let username = event.payload.username;
    let password_hash = event.payload.password_hash;
    println!("username {} password_hash {}", username, password_hash);

    // prepare the response
    let resp = Response {
        req_id: event.context.request_id,
        msg: format!("username {} pasword_hash {}", username, password_hash),
    };

    // return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
} */

/* #[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Deserialize, Serialize, Debug)]
struct CustomEvent {

}

pub(crate) async fn handler(event: LambdaEvent<CustomEvent>) -> Result<Value, LambdaError> {
    log::info!("{:?}", event);
    let uuid = Uuid::new_v4().to_string();
    Ok(json!({ "user_id": uuid.to_owned() }))
} */

/*
#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(hello)).await?;

    Ok(())
}

/// Sample pure Lambda function
async fn hello(_request: Request, _context: Context) -> Result<impl IntoResponse, Error> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body("Hello, World!".to_string())?)
}
*/

