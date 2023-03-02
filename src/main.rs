mod http_server;
mod core;
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env var must be set");
    http_server::launch(&db_url[..]).await
}
