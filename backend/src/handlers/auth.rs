use worker::*;
use crate::models::{LoginRequest, LoginResponse, ApiResponse};
use crate::db;
use crate::jwt;
use crate::utils::verify_password;

pub async fn login(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let body: LoginRequest = req.json().await?;
    
    let db = ctx.env.d1("DB")?;
    
    match db::get_user_by_username(&db, &body.username).await? {
        Some(user) => {
            if verify_password(&body.password, &user.password_hash) {
                // 使用 JWT 生成 token
                let token = jwt::create_jwt(user.id, &user.username)?;
                
                // Log this action
                let _ = db::add_log(&db, user.id, "login").await;
                
                let response = LoginResponse {
                    success: true,
                    token: Some(token),
                    message: "Login successful".to_string(),
                    user_id: Some(user.id),
                    username: Some(user.username),
                };
                Response::from_json(&response)
            } else {
                let response = LoginResponse {
                    success: false,
                    token: None,
                    message: "Invalid credentials".to_string(),
                    user_id: None,
                    username: None,
                };
                Response::from_json(&response).map(|r| r.with_status(401))
            }
        }
        None => {
            let response = LoginResponse {
                success: false,
                token: None,
                message: "User not found".to_string(),
                user_id: None,
                username: None,
            };
            Response::from_json(&response).map(|r| r.with_status(404))
        }
    }
}

pub async fn logout(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // JWT 是无状态的，客户端只需删除 token 即可
    // 如果需要服务端注销，可以维护一个 token 黑名单
    let response: ApiResponse<()> = ApiResponse::success(());
    Response::from_json(&response)
}

pub async fn check_session(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            // 验证用户是否仍然存在
            let db = ctx.env.d1("DB")?;
            match db::get_user_by_id(&db, claims.sub).await? {
                Some(_user) => {
                    let session_info = serde_json::json!({
                        "user_id": claims.sub,
                        "username": claims.username,
                        "expires_at": claims.exp,
                    });
                    let response = ApiResponse::success(session_info);
                    Response::from_json(&response)
                }
                None => {
                    let response: ApiResponse<()> = ApiResponse::error("User not found");
                    Response::from_json(&response).map(|r| r.with_status(401))
                }
            }
        }
        None => {
            let response: ApiResponse<()> = ApiResponse::error("Invalid or expired token");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}
