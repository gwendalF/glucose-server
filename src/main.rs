use services::Service;

mod config;
mod entities;
mod presentation;
mod repository;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let config = config::AppConfig::from_env().expect("Missing env variables");
    let app_state = Service::from_config(&config).await.unwrap();
    let app = presentation::get_router(app_state);
    axum::Server::bind(&config.server.server_url().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
