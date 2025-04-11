mod db;
mod api;

use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::setup_database().await.expect("Database setup failed");
    log::info!("Database setup complete.");

    // Run test query
    match db::run_test_query(&pool).await {
        Ok(_) => log::info!("Test query successful"),
        Err(e) => log::error!("Test query failed: {}", e),
    }

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

    log::info!("Starting server at {}:{}", host, port);
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(api::todo::AppState { pool: pool.clone() }))
            .configure(api::todo::config)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
