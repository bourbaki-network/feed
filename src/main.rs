mod config;
mod routes { pub mod health; }

use log;
use dotenv::dotenv;
use crate::routes::health::health;
use crate::config::get_config;
use actix_web::{App, HttpServer, middleware::Logger};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = get_config();

    std::env::set_var("RUST_LOG", config.get_string("log_level").unwrap());
    env_logger::init();

    log::info!("███████ ███████ ███████ ███████ ███████ ██████  ███████");
    log::info!("██      ██   ██ ██      ██      ██   ██ ██   ██ ██     ");
    log::info!("██      ███████ ███████ ██      ███████ ██   ██ █████  ");
    log::info!("██      ██   ██      ██ ██      ██   ██ ██   ██ ██     ");
    log::info!("███████ ██   ██ ███████ ███████ ██   ██ ██████  ███████");
    log::warn!(
        "Listening to host {} and port {}",
        config.get_string("host").unwrap(),
        config.get_int("port").unwrap()
    );

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(r#"%a %{r}a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#))
            .service(health)
    })
    .bind((
        config.get_string("host").unwrap(),
        config.get_int("port").unwrap() as u16
    ))?
    .run()
    .await
}
