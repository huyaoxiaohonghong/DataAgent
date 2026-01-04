mod auth;
mod db;
mod handlers;
mod jwt;
mod models;
mod utils;

use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Edge Management System API"))
        .post_async("/api/auth/login", handlers::auth::login)
        .post_async("/api/auth/logout", handlers::auth::logout)
        .get_async("/api/auth/check", handlers::auth::check_session)
        .get_async("/api/user/profile", handlers::user::get_profile)
        .get_async("/api/users", handlers::user::list_users)
        .post_async("/api/users", handlers::user::create_user)
        .delete_async("/api/users/:id", handlers::user::delete_user)
        .put_async("/api/users/:id", handlers::user::update_user)
        .get_async("/api/logs", handlers::logs::get_logs)
        // 代理节点管理
        .get_async("/api/proxy/nodes", handlers::proxy::list_nodes)
        .get_async("/api/proxy/nodes/:id", handlers::proxy::get_node)
        .post_async("/api/proxy/nodes", handlers::proxy::create_node)
        .put_async("/api/proxy/nodes/:id", handlers::proxy::update_node)
        .delete_async("/api/proxy/nodes/:id", handlers::proxy::delete_node)
        .post_async("/api/proxy/nodes/:id/check", handlers::proxy::check_node)
        .post_async("/api/proxy/nodes/check-all", handlers::proxy::check_all_nodes)
        .post_async("/api/proxy/import", handlers::proxy::import_subscription)
        .post_async("/api/proxy/nodes/batch-delete", handlers::proxy::batch_delete_nodes)
        .get_async("/api/proxy/subscriptions", handlers::proxy::list_subscription_info)
        .options("/api/*path", |_, _| {
            Response::empty()
                .map(|r| r.with_headers(utils::cors_headers()))
        })
        .run(req, env)
        .await
        .map(|res| {
            let mut headers = Headers::new();
            headers.set("Access-Control-Allow-Origin", "*").unwrap();
            headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS").unwrap();
            headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
            res.with_headers(headers)
        })
}
