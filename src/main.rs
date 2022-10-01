#![allow(clippy::needless_return)]

use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;
use serde::Serialize;
use serde_json::{json, Value};
// use surrealdb::Session;

const DB_URL: &str = "localhost:8000";

#[derive(Deserialize, Serialize)]
enum Route {
    SignUp,
    LogIn,
    LogOut,
    UserDelete,
    Invalid,
}

impl Route {
    fn from_str(str: &str) -> Self {
        match str {
            "SignUp" => Route::SignUp,
            "LogIn" => Route::LogIn,
            "LogOut" => Route::LogOut,
            "UserDelete" => Route::UserDelete,
            _ => Route::Invalid,
        }
    }
}

#[derive(Deserialize)]
struct CustomRequest {
    route: String,
    data: Value,
}

#[derive(Serialize)]
struct CustomResponse {
    route: Route,
    data: Value,
}

fn handle_sign_up(route: Route, req: Value) -> Result<Response<Body>, Error> {
    // let session = Session::for_kv();
    // println!("{:?}", session);
    let res = json!({
        "success": true,
        "req": req,
    });
    return build_response(route, res);
}

fn handle_log_in(route: Route, req: Value) -> Result<Response<Body>, Error> {
    let res = json!({
        "success": true,
        "req": req,
    });
    return build_response(route, res);
}

fn handle_log_out(route: Route, req: Value) -> Result<Response<Body>, Error> {
    let res = json!({
        "success": true,
        "req": req,
    });
    return build_response(route, res);
}

fn handle_user_delete(route: Route, req: Value) -> Result<Response<Body>, Error> {
    let res = json!({
        "success": true,
        "req": req,
    });
    return build_response(route, res);
}

fn handle_route(req: CustomRequest) -> Result<Response<Body>, Error> {
    let route = Route::from_str(req.route.as_str());
    println!("route = {}", req.route);
    return match route {
        Route::SignUp => handle_sign_up(route, req.data),
        Route::LogIn => handle_log_in(route, req.data),
        Route::LogOut => handle_log_out(route, req.data),
        Route::UserDelete => handle_user_delete(route, req.data),
        _ => build_err_response(route, "invalid route"),
    };
}

fn build_response(route: Route, data: Value) -> Result<Response<Body>, Error> {
    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let response = CustomResponse { route, data };
    let response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!(response).to_string().into())
        .map_err(Box::new)?;
    return Ok(response);
}

fn build_err_response(route: Route, err: &str) -> Result<Response<Body>, Error> {
    let response = CustomResponse {
        route,
        data: err.into(),
    };
    let response = Response::builder()
        .status(500)
        .header("Content-Type", "text/html")
        .body(json!(response).to_string().into())
        .map_err(Box::new)?;
    return Ok(response);
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
            return handle_route(req);
        }
        Body::Binary(payload) => {
            let req = serde_json::from_slice::<CustomRequest>(payload)?;
            return handle_route(req);
        }
        Body::Empty => {
            let resp = Response::builder()
                .status(200)
                .header("Content-Type", "text/html")
                .body("request body empty!".into())
                .map_err(Box::new)?;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_req() {
        use lambda_http::Request;
        let req = Request::default();
        println!("{:?}", req);
        assert!(true);
    }
}
