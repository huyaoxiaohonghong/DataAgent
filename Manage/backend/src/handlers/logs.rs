use worker::*;
use crate::models::{ApiResponse, LogEntry};
use crate::db;
use crate::jwt;

pub async fn get_logs(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(_claims) => {
            // Get limit from query params, default to 100
            let url = req.url()?;
            let limit: u32 = url.query_pairs()
                .find(|(k, _)| k == "limit")
                .and_then(|(_, v)| v.parse().ok())
                .unwrap_or(100);
            
            let logs = db::get_logs(&db, limit).await?;
            let response = ApiResponse::success(logs);
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<Vec<LogEntry>> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}
