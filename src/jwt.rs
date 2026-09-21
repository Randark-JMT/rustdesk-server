use hbb_common::log;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static SECRET: Lazy<String> =
    Lazy::new(|| crate::common::get_arg_or("RUSTDESK_API_JWT_KEY", String::new()));

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    user_id: u32,
    exp: usize,
}

pub fn generate_token(user_id: u32, exp: i64) -> Result<String, String> {
    let claims = Claims {
        user_id,
        exp: (chrono::Utc::now() + chrono::Duration::seconds(exp)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .map_err(|e| e.to_string())
}

pub fn verify_token(token: &str) -> Result<Claims, String> {
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            let now = chrono::Utc::now().timestamp() as usize;
            if token_data.claims.exp > now {
                Ok(token_data.claims)
            } else {
                log::debug!("JWT token expired");
                Err("Token status invalid or expired".to_string())
            }
        }
        Err(e) => {
            log::debug!("JWT verify failed: {}", e);
            Err("Invalid token".to_string())
        }
    }
}
