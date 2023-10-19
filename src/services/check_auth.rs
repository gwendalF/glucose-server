use super::AppState;
use crate::entities::ServerSecret;
use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sha1_smol::Digest;
use std::str::FromStr;

fn valid_secret(secret: &Digest, server_secret: &ServerSecret) -> bool {
    server_secret == secret
}

pub async fn auth_middleware<B>(
    State(service): State<AppState>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let mut authorized = false;
    if let Some(data) = request.headers().get("api-secret") {
        let digest = Digest::from_str(data.to_str().unwrap()).unwrap();
        authorized = valid_secret(&digest, &service.access_secret);
    }
    if authorized {
        let response = next.run(request).await;
        Ok(response)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
