use axum::{
    body::{to_bytes, Body, Bytes},
    http::Request,
    middleware::Next,
    response::Response,
};
use std::mem;
use tracing::info;

/// Routes where request and response bodies should not be logged
pub const BODY_BLACKLIST: &[&str] = &["/api/auth/register", "/api/auth/login"];

/// Middleware that logs requests and responses using `tracing`.
///
/// It logs headers and `x-request-id` for all requests. For routes not in
/// `BODY_BLACKLIST` it also logs the request and response bodies.
pub async fn logger(mut req: Request<Body>, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let method = req.method().to_string();
    let request_id = req
        .headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_owned();
    let headers = format!("{:?}", req.headers());

    let log_body = !BODY_BLACKLIST.iter().any(|p| p == &path);

    let req_body_string;
    if log_body {
        let body = mem::take(req.body_mut());
        let bytes = to_bytes(body, usize::MAX)
            .await
            .unwrap_or_else(|_| Bytes::new());
        req_body_string = String::from_utf8_lossy(&bytes).to_string();
        *req.body_mut() = Body::from(bytes.clone());
    } else {
        req_body_string = String::new();
    }

    let mut res = next.run(req).await;
    let status = res.status();

    let res_body_string;
    if log_body {
        let body = mem::take(res.body_mut());
        let bytes = to_bytes(body, usize::MAX)
            .await
            .unwrap_or_else(|_| Bytes::new());
        res_body_string = String::from_utf8_lossy(&bytes).to_string();
        *res.body_mut() = Body::from(bytes.clone());
    } else {
        res_body_string = String::new();
    }

    if log_body {
        info!(
            %request_id,
            method = %method,
            path = %path,
            status = %status,
            headers = %headers,
            req_body = %req_body_string,
            res_body = %res_body_string,
        );
    } else {
        info!(
            %request_id,
            method = %method,
            path = %path,
            status = %status,
            headers = %headers,
        );
    }

    res
}
