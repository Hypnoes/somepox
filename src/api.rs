use actix_web::{rt, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn hello(req: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", req);
    HttpResponse::Ok().body("Hello world!")
}

async fn log(req: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", req);
    HttpResponse::Ok().body("Hello world!")
}

async fn query(req: HttpRequest) -> impl Responder {
    println!("REQ: {:?}", req);
    HttpResponse::Ok().body("Hello world!")
}

pub fn api_server_init(end_point: String) -> std::io::Result<()> {
    rt::System::new().block_on(
        HttpServer::new(|| {
            App::new()
                .service(web::resource("/").route(web::get().to(hello)))
                .service(web::resource("/submit").route(web::post().to(log)))
                .service(web::resource("/query").route(web::get().to(query)))
        })
        .bind(end_point)?
        .run(),
    )
}
