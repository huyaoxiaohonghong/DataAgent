use worker::*;
use crate::models::Session;
use crate::utils::generate_token;

const SESSION_TTL_SECONDS: u64 = 86400; // 24 hours

pub async fn create_session(kv: &kv::KvStore, user_id: i64, username: &str) -> Result<String> {
    let token = generate_token();
    let now = js_sys::Date::now() as i64 / 1000;
    
    let session = Session {
        user_id,
        username: username.to_string(),
        created_at: now,
        expires_at: now + SESSION_TTL_SECONDS as i64,
    };
    
    let session_json = serde_json::to_string(&session)
        .map_err(|e| Error::RustError(e.to_string()))?;
    
    kv.put(&format!("session:{}", token), session_json)?
        .expiration_ttl(SESSION_TTL_SECONDS)
        .execute()
        .await?;
    
    Ok(token)
}

pub async fn get_session(kv: &kv::KvStore, token: &str) -> Result<Option<Session>> {
    let session_json = kv.get(&format!("session:{}", token)).text().await?;
    
    match session_json {
        Some(json) => {
            let session: Session = serde_json::from_str(&json)
                .map_err(|e| Error::RustError(e.to_string()))?;
            Ok(Some(session))
        }
        None => Ok(None),
    }
}

pub async fn delete_session(kv: &kv::KvStore, token: &str) -> Result<()> {
    kv.delete(&format!("session:{}", token)).await?;
    Ok(())
}
