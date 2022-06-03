use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
mod things;
mod tool;

#[get("/{name}")]
async fn hello_name(web::Path(name): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name))
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // show some tests with rust
    things::things();
    tool::new_thing("Http server");

    // now start the webserver
    let addr = "127.0.0.1";
    let port = "8080";
    let url = format!("{}:{}", addr, port);

    println!("Run on http://{}", url);
    println!("Try http://{}/John 💻", url);

    HttpServer::new(|| App::new().service(index).service(hello_name))
        .bind(url)?
        .run()
        .await
}
