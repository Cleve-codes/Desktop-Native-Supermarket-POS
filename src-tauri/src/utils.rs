use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::error::{AppError, AppResult};
use uuid::Uuid;
use chrono::Utc;
use serde::{Serialize, Deserialize};

// Password hashing with Argon2
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Other(e.to_string()))?
        .to_string();
    
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::Other(e.to_string()))?;
    
    let argon2 = Argon2::default();
    let is_valid = argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok();
    
    Ok(is_valid)
}

// Simple JWT-like token for authentication
#[derive(Serialize, Deserialize)]
struct TokenClaims {
    sub: String,  // subject (user id)
    exp: i64,     // expiration time
    iat: i64,     // issued at
    role: String, // user role
}

pub fn generate_token(user_id: &str, role: &str) -> String {
    let claims = TokenClaims {
        sub: user_id.to_string(),
        exp: (Utc::now().timestamp() + 86400), // 24 hour expiry
        iat: Utc::now().timestamp(),
        role: role.to_string(),
    };
    
    // In a real implementation, this would be encrypted/signed properly
    // Here we're just doing a simple JSON encoding with a prefix
    let token_data = serde_json::to_string(&claims).unwrap_or_default();
    format!("POS.{}.{}", base64_encode(&token_data), Uuid::new_v4())
}

pub fn validate_token(token: &str) -> AppResult<(String, String)> {
    if !token.starts_with("POS.") {
        return Err(AppError::Authentication("Invalid token format".to_string()));
    }
    
    let parts: Vec<&str> = token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err(AppError::Authentication("Invalid token format".to_string()));
    }
    
    let token_data = base64_decode(parts[1])
        .map_err(|_| AppError::Authentication("Invalid token data".to_string()))?;
    
    let claims: TokenClaims = serde_json::from_str(&token_data)
        .map_err(|_| AppError::Authentication("Invalid token data".to_string()))?;
    
    // Check expiration
    if claims.exp < Utc::now().timestamp() {
        return Err(AppError::Authentication("Token expired".to_string()));
    }
    
    Ok((claims.sub, claims.role))
}

// Simple base64 encoding/decoding for our token
fn base64_encode(input: &str) -> String {
    base64::encode(input)
}

fn base64_decode(input: &str) -> Result<String, base64::DecodeError> {
    let bytes = base64::decode(input)?;
    String::from_utf8(bytes).map_err(|_| base64::DecodeError::InvalidLength)
} 