mod api;
mod collections;
mod server;
mod jwt;

// serving address of your api
pub const API_ADDR: &str = "0.0.0.0:8080";
// mongo db related
pub const MONGO_ADDR: &str = "mongodb://root:example@mongo:27017";
pub const MONGO_DB_NAME: &str = "db_name";

#[actix_web::main]
pub async fn main() -> anyhow::Result<()> {
    crate::server::start_server().await?;
    Ok(())
}
