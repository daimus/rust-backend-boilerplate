use std::env;
use axum::body::Body;
use axum::http;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use headers::{Authorization, Header};
use headers::authorization::Bearer;
use hyper::StatusCode;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::get;
use serde::{Deserialize, Serialize};
use crate::infrastructure::presenter::http::response::ApiResponseBuilder;

#[derive(Debug, Serialize, Deserialize)]
struct Jwks {
    keys: Vec<Jwk>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Jwk {
    kid: String,
    alg: String,
    kty: String,
    n: String,
    e: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iss: String
}

// Fetch JWKS from provider
async fn fetch_jwks() -> Result<Jwks, reqwest::Error> {
    let res = get(env::var("JWKS_URL").expect("JWKS_URL must be set")).await?.json::<Jwks>().await?;
    Ok(res)
}

// Find public key by KID
fn get_public_key(jwks: &Jwks, kid: &str) -> Option<DecodingKey> {
    jwks.keys.iter().find(|k| k.kid == kid).map(|key| {
        DecodingKey::from_rsa_components(&key.n, &key.e).expect("Invalid RSA key")
    })
}

// Validate JWT
async fn validate_jwt(token: &str) -> Result<Claims, String> {
    let header = decode_header(token).map_err(|e| e.to_string())?;
    let kid = header.kid.ok_or("No KID in token header")?;

    let jwks = fetch_jwks().await.map_err(|e| e.to_string())?;
    let decoding_key = get_public_key(&jwks, &kid).ok_or("No matching key found")?;

    let mut validation = Validation::new(Algorithm::RS256);
    let issuer = env::var("JWT_ISSUER").expect("JWT_ISSUER must be set");
    validation.set_issuer(&[issuer]);
    validation.validate_aud = false;

    decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| e.to_string())
}

pub async fn auth(
    mut req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let mut headers = req
        .headers_mut()
        .iter()
        .filter_map(|(header_name, header_value)| {
            if header_name == http::header::AUTHORIZATION {
                return Some(header_value);
            }
            None
        });

    let header : Authorization<Bearer> = match Authorization::decode(&mut headers) {
        Ok(h) => h,
        Err(_) => return ApiResponseBuilder::new("Invalid Token").status_code(StatusCode::UNAUTHORIZED).build().into_response(),
    };
    let token = header.token().to_string();
    match validate_jwt(&*token).await {
        Ok(_) => {
            next.run(req).await
        }
        Err(e) => ApiResponseBuilder::new(e).status_code(StatusCode::UNAUTHORIZED).build().into_response(),
    }
}