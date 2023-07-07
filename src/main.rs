use lambda_http::{
    ext::PayloadError,
    http::{header::CONTENT_TYPE, Method},
    run, service_fn, Body, Error, Request, RequestPayloadExt, Response,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default, Serialize)]
struct CommentParams {
    pub url: String,
    pub body: String,
    pub signature: String,
    pub email: String,
    pub website: Option<String>,
}

fn parse_comment_params(request_body: impl Into<String>) -> Result<CommentParams, Error> {
    let result = serde_json::from_str(&request_body.into()).unwrap();
    Ok(result)
}

fn handle_get_request(event: Request) -> Result<Response<Body>, Error> {
    unimplemented!()
}

fn verify_content_type(event: &Request) -> bool {
    match event.headers().get(CONTENT_TYPE) {
        None => false,
        Some(value) => match value.to_str() {
            Ok(value) => value.starts_with("application/json"),
            _ => false,
        },
    }
}

fn handle_post_request(event: Request) -> Result<Response<Body>, Error> {
    if !verify_content_type(&event) {
        return handle_bad_request();
    }

    let body: Option<CommentParams> = event.payload()?;
    if body.is_none() {
        return handle_bad_request();
    }

    let body = body.unwrap();
    let response_body = serde_json::to_string(&body).unwrap();

    let resp = Response::builder()
        .status(200)
        .body(response_body.into())
        .unwrap();
    dbg!(body.url);
    Ok(resp)
}

fn handle_bad_request() -> Result<Response<Body>, Error> {
    let resp = Response::builder()
        .status(400)
        .body("Bad request".into())
        .unwrap();
    Ok(resp)
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    match event.method() {
        &Method::GET => handle_get_request(event),
        &Method::POST => handle_post_request(event),
        _ => handle_bad_request(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
