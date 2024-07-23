use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use log::info;
use uuid::Uuid;

pub async fn mw_map_response(uri: Uri, req_method: Method, res: Response) -> Response {
    let uuid = Uuid::new_v4();
    info!("---> MAP RESPONSE");
    info!("Uuid: {:?}", uuid);
    info!("Request method: {:?}", req_method);
    info!("Request URI: {:?}", uri);

    (StatusCode::ACCEPTED, res).into_response()
}
