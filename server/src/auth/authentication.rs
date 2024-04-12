use jsonwebtoken::{decode, decode_header, encode, DecodingKey, EncodingKey, Header, Validation};
use reqwest;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize)]
pub struct SessionClaims {
    id: String,
    role: String,
    profile_pic_url: String,
}

pub async fn create_token(
    profile_pic_url: String,
    id: String,
    role: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let claims = SessionClaims {
        id,
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

pub async fn decode_jwt(
    token: String,
) -> Result<(String, String, String, Option<String>), Box<dyn std::error::Error>> {
    let header = decode_header(&token)?;
    let kid = header.kid.ok_or("Error unwrapping kid")?;
    let key = validate_token(kid).await?;
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

pub async fn validate_token(kid: String) -> Result<RsaKey, Box<dyn std::error::Error>> {
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
struct RsaKey {
    // alg: String,
    // r#use: String,
    // kty: String,
    kid: String,
    e: String,
    n: String,
}
