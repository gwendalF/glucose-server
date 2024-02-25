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
    let address = &config.server.server_url();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
