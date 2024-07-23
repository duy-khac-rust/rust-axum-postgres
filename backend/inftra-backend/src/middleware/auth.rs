use axum::{extract::Request, middleware::Next, response::Response};
use log::info;
// use tracing::debug;


pub async fn mw_auth(req: Request, next: Next) -> Response {
  info!("->> MIDDLEWARE AUTH");
  
  next.run(req).await
}
