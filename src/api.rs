use std::{fmt, process::id, sync::mpsc::Sender};

use actix_web::{rt, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct LogRequest {
    content: Option<String>,
}

impl fmt::Display for LogRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "log content: {:?}", self.content)
    }
}

#[derive(Serialize, Deserialize)]
struct QueryRequest {
    id: u64,
}

impl fmt::Display for QueryRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "query: QueryRequest (id : {}) ", self.id)
    }
}

pub enum CmdType {
    Log(String),
    Query(u64),
}

pub fn api_server_init(end_point: String, tx: Sender<CmdType>) -> Result<()> {
    rt::System::new()
        .block_on(
            HttpServer::new(move || {
                App::new()
                    .app_data(web::Data::new(tx.clone()))
                    .service(web::resource("/").route(web::get().to(hello)))
                    .service(web::resource("/submit").route(web::post().to(log)))
                    .service(web::resource("/query").route(web::get().to(query)))
            })
            .bind(end_point)?
            .run(),
        )
        .map_err(|_| anyhow!("Error."))
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn log(log_req: web::Json<LogRequest>, data: web::Data<Sender<CmdType>>) -> impl Responder {
    println!("REQ: {}", log_req);
    if let Some(content) = &log_req.content {
        let _ = data.send(CmdType::Log(content.clone()));
    }
    HttpResponse::Ok().body("Log Send.")
}

async fn query(
    query_req: web::Query<QueryRequest>,
    data: web::Data<Sender<CmdType>>,
) -> impl Responder {
    println!("query: {}", query_req);
    // data.send(format!("query:{}", query_req));
    let _ = data.send(CmdType::Query(query_req.id));
    HttpResponse::Ok().body("Hello world!")
}
