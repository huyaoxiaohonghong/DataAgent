use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>,
    pub message: String,
    pub user_id: Option<i64>,
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub user_id: i64,
    pub username: String,
    pub created_at: i64,
    pub expires_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub id: i64,
    pub user_id: i64,
    pub action: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            success: false,
            data: None,
            message: message.to_string(),
        }
    }
}

// 代理节点模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyNode {
    pub id: i64,
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub extra_config: Option<String>,
    pub group_name: Option<String>,
    pub status: String,
    pub latency: Option<i64>,
    pub last_check_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProxyNodeRequest {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub extra_config: Option<String>,
    pub group_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProxyNodeRequest {
    pub name: String,
    pub protocol: String,
    pub address: String,
    pub port: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub extra_config: Option<String>,
    pub group_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyCheckResult {
    pub id: i64,
    pub status: String,
    pub latency: Option<i64>,
    pub message: String,
}
