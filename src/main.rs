mod server;
mod config;
mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}