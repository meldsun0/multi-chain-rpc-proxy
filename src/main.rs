mod server;
mod config;
mod model;
mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}