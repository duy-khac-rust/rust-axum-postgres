use api_backend::routes;
use axum::{middleware, Router};
use core_backend::config::ProdConfig;
use dotenv::dotenv;
use inftra_backend::{
    init_db,
    middleware::{auth::mw_auth, ctx_response::mw_map_response},
};
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // env_logger::init();

    let cfg = ProdConfig::from_env()
        .await
        .expect("Get value from environment failed");
    // info!("{:#?}", cfg);

    // Connect to database

    let db = init_db(cfg.postgres.dsn.as_str(), cfg.postgres.max_connect)
        .await
        .expect("Connect to database failed!");
    info!("Connected to database successfully");

    // Start server

    let app = Router::new()
        .merge(routes())
        .layer(middleware::map_response(mw_map_response))
        .layer(middleware::from_fn_with_state(db.clone(), mw_auth))
        .with_state(db);

    let listener = TcpListener::bind(cfg.web.port)
        .await
        .expect("Connect to web server failed");

    info!("Starting server");
    axum::serve(listener, app).await.unwrap();
}
