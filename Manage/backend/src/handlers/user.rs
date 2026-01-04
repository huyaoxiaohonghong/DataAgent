use worker::*;
use crate::models::{CreateUserRequest, UpdateUserRequest, ApiResponse, User};
use crate::db;
use crate::jwt;
use crate::utils::hash_password;

pub async fn get_profile(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            match db::get_user_by_id(&db, claims.sub).await? {
                Some(user) => {
                    let response = ApiResponse::success(user);
                    Response::from_json(&response)
                }
                None => {
                    let response: ApiResponse<User> = ApiResponse::error("User not found");
                    Response::from_json(&response).map(|r| r.with_status(404))
                }
            }
        }
        None => {
            let response: ApiResponse<User> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn list_users(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(_claims) => {
            let users = db::list_users(&db).await?;
            let response = ApiResponse::success(users);
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<Vec<User>> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn create_user(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            let body: CreateUserRequest = req.json().await?;
            let password_hash = hash_password(&body.password);
            
            db::create_user(&db, &body.username, &password_hash).await?;
            
            // Log this action
            let _ = db::add_log(&db, claims.sub, &format!("created user: {}", body.username)).await;
            
            let response: ApiResponse<()> = ApiResponse::success(());
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<()> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn delete_user(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let user_id: i64 = ctx.param("id")
        .ok_or_else(|| Error::RustError("Missing user id".to_string()))?
        .parse()
        .map_err(|_| Error::RustError("Invalid user id".to_string()))?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            // Prevent deleting yourself
            if claims.sub == user_id {
                let response: ApiResponse<()> = ApiResponse::error("Cannot delete yourself");
                return Response::from_json(&response).map(|r| r.with_status(400));
            }
            
            db::delete_user(&db, user_id).await?;
            
            // Log this action
            let _ = db::add_log(&db, claims.sub, &format!("deleted user id: {}", user_id)).await;
            
            let response: ApiResponse<()> = ApiResponse::success(());
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<()> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn update_user(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let user_id: i64 = ctx.param("id")
        .ok_or_else(|| Error::RustError("Missing user id".to_string()))?
        .parse()
        .map_err(|_| Error::RustError("Invalid user id".to_string()))?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            let body: UpdateUserRequest = req.json().await?;
            
            let password_hash = body.password
                .filter(|p| !p.is_empty())
                .map(|p| hash_password(&p));
            
            db::update_user(&db, user_id, &body.username, password_hash.as_deref()).await?;
            
            // Log this action
            let _ = db::add_log(&db, claims.sub, &format!("updated user id: {}", user_id)).await;
            
            let response: ApiResponse<()> = ApiResponse::success(());
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<()> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}
