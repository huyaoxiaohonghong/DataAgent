use worker::*;
use worker::d1::D1Database;
use crate::models::User;

pub async fn get_user_by_username(db: &D1Database, username: &str) -> Result<Option<User>> {
    let stmt = db.prepare("SELECT id, username, password_hash, created_at FROM users WHERE username = ?");
    let query = stmt.bind(&[username.into()])?;
    
    let result = query.first::<User>(None).await?;
    Ok(result)
}

pub async fn get_user_by_id(db: &D1Database, user_id: i64) -> Result<Option<User>> {
    let query = format!("SELECT id, username, password_hash, created_at FROM users WHERE id = {}", user_id);
    let stmt = db.prepare(&query);
    
    let result = stmt.first::<User>(None).await?;
    Ok(result)
}

pub async fn create_user(db: &D1Database, username: &str, password_hash: &str) -> Result<()> {
    let stmt = db.prepare("INSERT INTO users (username, password_hash, created_at) VALUES (?, ?, datetime('now'))");
    stmt.bind(&[username.into(), password_hash.into()])?
        .run()
        .await?;
    Ok(())
}

pub async fn list_users(db: &D1Database) -> Result<Vec<User>> {
    let stmt = db.prepare("SELECT id, username, password_hash, created_at FROM users ORDER BY id");
    let result = stmt.all().await?;
    
    let users: Vec<User> = result.results()?;
    Ok(users)
}

pub async fn add_log(db: &D1Database, user_id: i64, action: &str) -> Result<()> {
    let query = format!("INSERT INTO logs (user_id, action, timestamp) VALUES ({}, ?, datetime('now'))", user_id);
    let stmt = db.prepare(&query);
    stmt.bind(&[action.into()])?
        .run()
        .await?;
    Ok(())
}

pub async fn get_logs(db: &D1Database, limit: u32) -> Result<Vec<crate::models::LogEntry>> {
    let query = format!("SELECT id, user_id, action, timestamp FROM logs ORDER BY id DESC LIMIT {}", limit);
    let stmt = db.prepare(&query);
    let result = stmt.all().await?;
    
    let logs: Vec<crate::models::LogEntry> = result.results()?;
    Ok(logs)
}

pub async fn delete_user(db: &D1Database, user_id: i64) -> Result<()> {
    let query = format!("DELETE FROM users WHERE id = {}", user_id);
    let stmt = db.prepare(&query);
    stmt.run().await?;
    Ok(())
}

pub async fn update_user(db: &D1Database, user_id: i64, username: &str, password_hash: Option<&str>) -> Result<()> {
    match password_hash {
        Some(hash) => {
            let query = format!("UPDATE users SET username = ?, password_hash = ? WHERE id = {}", user_id);
            let stmt = db.prepare(&query);
            stmt.bind(&[username.into(), hash.into()])?
                .run()
                .await?;
        }
        None => {
            let query = format!("UPDATE users SET username = ? WHERE id = {}", user_id);
            let stmt = db.prepare(&query);
            stmt.bind(&[username.into()])?
                .run()
                .await?;
        }
    }
    Ok(())
}

// ============= 代理节点操作 =============

pub async fn list_proxy_nodes(db: &D1Database) -> Result<Vec<crate::models::ProxyNode>> {
    let stmt = db.prepare("SELECT id, name, protocol, address, port, username, password, extra_config, group_name, status, latency, last_check_at, created_at, updated_at FROM proxy_nodes ORDER BY id DESC");
    let result = stmt.all().await?;
    
    let nodes: Vec<crate::models::ProxyNode> = result.results()?;
    Ok(nodes)
}

pub async fn get_proxy_node(db: &D1Database, id: i64) -> Result<Option<crate::models::ProxyNode>> {
    let query = format!("SELECT id, name, protocol, address, port, username, password, extra_config, group_name, status, latency, last_check_at, created_at, updated_at FROM proxy_nodes WHERE id = {}", id);
    let stmt = db.prepare(&query);
    let result = stmt.first::<crate::models::ProxyNode>(None).await?;
    Ok(result)
}

pub async fn create_proxy_node(
    db: &D1Database,
    name: &str,
    protocol: &str,
    address: &str,
    port: i64,
    username: Option<&str>,
    password: Option<&str>,
    extra_config: Option<&str>,
    group_name: Option<&str>,
) -> Result<i64> {
    let stmt = db.prepare(
        "INSERT INTO proxy_nodes (name, protocol, address, port, username, password, extra_config, group_name, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'unknown', datetime('now'), datetime('now'))"
    );
    
    let port_str = port.to_string();
    stmt.bind(&[
        name.into(),
        protocol.into(),
        address.into(),
        port_str.as_str().into(),
        username.unwrap_or("").into(),
        password.unwrap_or("").into(),
        extra_config.unwrap_or("").into(),
        group_name.unwrap_or("").into(),
    ])?
    .run()
    .await?;
    
    // 获取最后插入的ID
    let last_id_stmt = db.prepare("SELECT last_insert_rowid() as id");
    let result = last_id_stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(val) => {
            let id = val.get("id").and_then(|v| v.as_i64()).unwrap_or(0);
            Ok(id)
        }
        None => Ok(0)
    }
}

pub async fn update_proxy_node(
    db: &D1Database,
    id: i64,
    name: &str,
    protocol: &str,
    address: &str,
    port: i64,
    username: Option<&str>,
    password: Option<&str>,
    extra_config: Option<&str>,
    group_name: Option<&str>,
) -> Result<()> {
    let query = format!(
        "UPDATE proxy_nodes SET name = ?, protocol = ?, address = ?, port = {}, username = ?, password = ?, extra_config = ?, group_name = ?, updated_at = datetime('now') WHERE id = {}",
        port, id
    );
    let stmt = db.prepare(&query);
    stmt.bind(&[
        name.into(),
        protocol.into(),
        address.into(),
        username.unwrap_or("").into(),
        password.unwrap_or("").into(),
        extra_config.unwrap_or("").into(),
        group_name.unwrap_or("").into(),
    ])?
    .run()
    .await?;
    Ok(())
}

pub async fn delete_proxy_node(db: &D1Database, id: i64) -> Result<()> {
    let query = format!("DELETE FROM proxy_nodes WHERE id = {}", id);
    let stmt = db.prepare(&query);
    stmt.run().await?;
    Ok(())
}

pub async fn update_proxy_node_status(db: &D1Database, id: i64, status: &str, latency: Option<i64>) -> Result<()> {
    let latency_str = latency.map(|l| l.to_string()).unwrap_or_else(|| "NULL".to_string());
    let query = format!(
        "UPDATE proxy_nodes SET status = ?, latency = {}, last_check_at = datetime('now'), updated_at = datetime('now') WHERE id = {}",
        latency_str, id
    );
    let stmt = db.prepare(&query);
    stmt.bind(&[status.into()])?
        .run()
        .await?;
    Ok(())
}

pub async fn batch_delete_proxy_nodes(db: &D1Database, ids: &[i64]) -> Result<usize> {
    if ids.is_empty() {
        return Ok(0);
    }
    
    let id_list = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
    let query = format!("DELETE FROM proxy_nodes WHERE id IN ({})", id_list);
    let stmt = db.prepare(&query);
    stmt.run().await?;
    Ok(ids.len())
}
