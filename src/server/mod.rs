mod query;

use actix_web::{post, web, App, HttpServer, Responder, Result};
use query::query as query_handler;
use serde::{Deserialize, Serialize};

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(query_handler))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
