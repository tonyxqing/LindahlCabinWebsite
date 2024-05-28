use std::sync::Arc;

use async_graphql::Context;
use jsonwebtoken::{decode, decode_header, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::oid::ObjectId;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::{db::UserFilter, gql::Resolver, model::Role};

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleHeader {
    alg: String,
    kid: String,
    typ: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleClaims {
    iss: Option<String>,
    azp: Option<String>,
    aud: Option<String>,
    sub: String,
    email: String,
    email_verified: Option<bool>,
    nbf: Option<i32>,
    name: String,
    picture: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    locale: Option<String>,
    iat: Option<i32>,
    exp: Option<i32>,
    jti: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionClaims {
    sub: String,
    name: String,
    exp: i64,
    role: String,
    profile_pic_url: String,
}

pub async fn create_token(
    profile_pic_url: String,
    sub: String,
    role: String,
    name: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let claims = SessionClaims {
        sub,
        name,
        exp: chrono::Utc::now()
            .checked_add_signed(chrono::Duration::weeks(2))
            .expect("Failed to calculate token expiration")
            .timestamp(),
        role,
        profile_pic_url,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )?;
    Ok(token)
}

pub fn token_from_header(http_req: &actix_web::HttpRequest) -> Option<String> {
    http_req
        .headers()
        .get("Authorization")
        .and_then(|v| match v.to_str().ok() {
            None => None,
            Some(s) => {
                token_from_bearer_string(s.to_string())},
        })
}

pub fn token_from_bearer_string(s: String) -> Option<String> {
    if s.starts_with("Bearer ") {
        let jwt_start_index = "Bearer ".len();
        let jwt = s[jwt_start_index..s.len()].to_string();
        Some(jwt)
    } else {
        None
    }
}

#[derive(Debug, Default, Clone)]
pub struct AuthResult {
    pub id: String,
    pub admin: bool,
    pub owner: bool,
    pub member: bool,
}

impl AuthResult {
    pub fn from_context(ctx: &Context<'_>, role: Role) -> Result<String, String> {
        let result = ctx.data::<AuthResult>().expect("Auth Result not found in this request");
        let pass = match role {
            Role::Admin => result.admin,
            Role::Member => result.member,
            Role::Owner => result.owner,
        };
        if pass {
            Ok(result.id.clone())
        } else {
            Err("Failed to meet role requirement".to_string())
        }
    }
}

pub async fn resolve_token(token: Option<String>, r: Arc<Resolver>) -> Result<Option<AuthResult>, String> {
    if token.is_none() {
        return Ok(None)
    }
    let claims = decode::<SessionClaims>(
        &token.unwrap(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|e| e.to_string())?
    .claims;

    let account_id = ObjectId::parse_str(claims.clone().sub).map_err(|e| e.to_string())?;
    let user = r.get_users(UserFilter {
        id: Some(account_id),
        ..Default::default()
    }).await?;
    if user.is_empty() {
       return Err("Token didn't resolve to account".to_string())
    }

    Ok(Some(AuthResult {
        id: claims.sub,
        admin: user.first().unwrap().role ==  Role::Admin,
        owner: user.first().unwrap().role == Role::Owner,
        member: user.first().unwrap().role == Role::Member
    }))
}

pub async fn decode_jwt(
    token: String,
) -> Result<(String, String, String, Option<String>), Box<dyn std::error::Error>> {
    let header = decode_header(&token)?;
    let kid = header.kid.ok_or("Error unwrapping kid")?;
    let key = get_google_key(kid).await?;
    let claims = decode::<GoogleClaims>(
        &token,
        &DecodingKey::from_rsa_components(&key.n, &key.e)?,
        &{
            let mut v = Validation::new(jsonwebtoken::Algorithm::from(header.alg));
            v.set_audience(&[
                "130478330472-mar4k4d0kea019930om0m7m0elpoju6o.apps.googleusercontent.com",
            ]);
            v.validate_exp = false;
            v
        },
    )?
    .claims;
    Ok((claims.sub, claims.email, claims.name, claims.picture))
}

pub async fn get_google_key(kid: String) -> Result<RsaKey, Box<dyn std::error::Error>> {
    let certs = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await?
        .json::<RsaSet>()
        .await?;
    let key = certs
        .keys
        .iter()
        .find(|key| key.kid.eq(&kid))
        .ok_or("Unable to find kid in Google rsa set".to_string())?;

    Ok(key.clone())
}

pub async fn verify_user() {
    todo!()
}
#[derive(Deserialize)]
struct RsaSet {
    keys: Vec<RsaKey>,
}

#[derive(Deserialize, Clone)]
pub struct RsaKey {
    // alg: String,
    // r#use: String,
    // kty: String,
    kid: String,
    e: String,
    n: String,
}
