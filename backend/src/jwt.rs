use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use worker::*;

type HmacSha256 = Hmac<Sha256>;

// JWT 密钥 - 生产环境应从环境变量读取
const JWT_SECRET: &[u8] = b"edge-management-secret-key-2024";
const JWT_EXPIRATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,      // user_id
    pub username: String,
    pub exp: i64,      // expiration time
    pub iat: i64,      // issued at
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtHeader {
    alg: String,
    typ: String,
}

pub fn create_jwt(user_id: i64, username: &str) -> Result<String> {
    let now = (js_sys::Date::now() / 1000.0) as i64;
    let exp = now + (JWT_EXPIRATION_HOURS * 3600);
    
    let header = JwtHeader {
        alg: "HS256".to_string(),
        typ: "JWT".to_string(),
    };
    
    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp,
        iat: now,
    };
    
    // Encode header and payload
    let header_json = serde_json::to_string(&header)
        .map_err(|e| Error::RustError(format!("Header serialize error: {}", e)))?;
    let claims_json = serde_json::to_string(&claims)
        .map_err(|e| Error::RustError(format!("Claims serialize error: {}", e)))?;
    
    let header_b64 = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
    let claims_b64 = URL_SAFE_NO_PAD.encode(claims_json.as_bytes());
    
    let message = format!("{}.{}", header_b64, claims_b64);
    
    // Sign with HMAC-SHA256
    let mut mac = HmacSha256::new_from_slice(JWT_SECRET)
        .map_err(|e| Error::RustError(format!("HMAC key error: {}", e)))?;
    mac.update(message.as_bytes());
    let signature = mac.finalize().into_bytes();
    let signature_b64 = URL_SAFE_NO_PAD.encode(&signature);
    
    Ok(format!("{}.{}", message, signature_b64))
}

pub fn verify_jwt(token: &str) -> Result<Claims> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(Error::RustError("Invalid JWT format".to_string()));
    }
    
    let header_b64 = parts[0];
    let claims_b64 = parts[1];
    let signature_b64 = parts[2];
    
    // Verify signature
    let message = format!("{}.{}", header_b64, claims_b64);
    let mut mac = HmacSha256::new_from_slice(JWT_SECRET)
        .map_err(|e| Error::RustError(format!("HMAC key error: {}", e)))?;
    mac.update(message.as_bytes());
    
    let signature = URL_SAFE_NO_PAD.decode(signature_b64)
        .map_err(|_| Error::RustError("Invalid signature encoding".to_string()))?;
    
    mac.verify_slice(&signature)
        .map_err(|_| Error::RustError("Invalid signature".to_string()))?;
    
    // Decode claims
    let claims_bytes = URL_SAFE_NO_PAD.decode(claims_b64)
        .map_err(|_| Error::RustError("Invalid claims encoding".to_string()))?;
    let claims: Claims = serde_json::from_slice(&claims_bytes)
        .map_err(|e| Error::RustError(format!("Claims parse error: {}", e)))?;
    
    // Check expiration
    let now = (js_sys::Date::now() / 1000.0) as i64;
    if claims.exp < now {
        return Err(Error::RustError("Token expired".to_string()));
    }
    
    Ok(claims)
}

pub fn extract_jwt_claims(req: &Request) -> Option<Claims> {
    let auth_header = req.headers().get("Authorization").ok()??;
    let token = auth_header.strip_prefix("Bearer ")?;
    verify_jwt(token).ok()
}
