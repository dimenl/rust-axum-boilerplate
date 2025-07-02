// Request guards placeholder

use axum::{http::Request, middleware::Next, response::Response};

pub async fn auth_guard<B>(req: Request<B>, next: Next<B>) -> Response {
    // TODO: implement guard logic
    next.run(req).await
}
