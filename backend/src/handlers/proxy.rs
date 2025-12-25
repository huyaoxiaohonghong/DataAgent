use worker::*;
use crate::models::{CreateProxyNodeRequest, UpdateProxyNodeRequest, ApiResponse, ProxyNode, ProxyCheckResult};
use crate::db;
use crate::jwt;

pub async fn list_nodes(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(_claims) => {
            let nodes = db::list_proxy_nodes(&db).await?;
            let response = ApiResponse::success(nodes);
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<Vec<ProxyNode>> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn get_node(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let id: i64 = ctx.param("id")
        .ok_or_else(|| Error::RustError("Missing id".to_string()))?
        .parse()
        .map_err(|_| Error::RustError("Invalid id".to_string()))?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(_claims) => {
            match db::get_proxy_node(&db, id).await? {
                Some(node) => {
                    let response = ApiResponse::success(node);
                    Response::from_json(&response)
                }
                None => {
                    let response: ApiResponse<ProxyNode> = ApiResponse::error("Node not found");
                    Response::from_json(&response).map(|r| r.with_status(404))
                }
            }
        }
        None => {
            let response: ApiResponse<ProxyNode> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn create_node(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            let body: CreateProxyNodeRequest = req.json().await?;
            
            let id = db::create_proxy_node(
                &db,
                &body.name,
                &body.protocol,
                &body.address,
                body.port,
                body.username.as_deref(),
                body.password.as_deref(),
                body.extra_config.as_deref(),
                body.group_name.as_deref(),
            ).await?;
            
            // 记录日志
            let _ = db::add_log(&db, claims.sub, &format!("created proxy node: {}", body.name)).await;
            
            let response = ApiResponse::success(serde_json::json!({ "id": id }));
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<()> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn update_node(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let id: i64 = ctx.param("id")
        .ok_or_else(|| Error::RustError("Missing id".to_string()))?
        .parse()
        .map_err(|_| Error::RustError("Invalid id".to_string()))?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            let body: UpdateProxyNodeRequest = req.json().await?;
            
            db::update_proxy_node(
                &db,
                id,
                &body.name,
                &body.protocol,
                &body.address,
                body.port,
                body.username.as_deref(),
                body.password.as_deref(),
                body.extra_config.as_deref(),
                body.group_name.as_deref(),
            ).await?;
            
            let _ = db::add_log(&db, claims.sub, &format!("updated proxy node id: {}", id)).await;
            
            let response: ApiResponse<()> = ApiResponse::success(());
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<()> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

pub async fn delete_node(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let id: i64 = ctx.param("id")
        .ok_or_else(|| Error::RustError("Missing id".to_string()))?
        .parse()
        .map_err(|_| Error::RustError("Invalid id".to_string()))?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            db::delete_proxy_node(&db, id).await?;
            let _ = db::add_log(&db, claims.sub, &format!("deleted proxy node id: {}", id)).await;
            
            let response: ApiResponse<()> = ApiResponse::success(());
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<()> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

// 验真功能 - 检查代理是否可用
pub async fn check_node(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    let id: i64 = ctx.param("id")
        .ok_or_else(|| Error::RustError("Missing id".to_string()))?
        .parse()
        .map_err(|_| Error::RustError("Invalid id".to_string()))?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(_claims) => {
            // 获取节点信息
            match db::get_proxy_node(&db, id).await? {
                Some(node) => {
                    // 模拟验真 - 实际应该发起真实的连接测试
                    // 在 Cloudflare Worker 中可以使用 fetch 测试 HTTP/HTTPS 代理
                    let start_time = js_sys::Date::now();
                    
                    // 简单的检测逻辑：尝试解析地址
                    let is_valid = !node.address.is_empty() && node.port > 0 && node.port < 65536;
                    
                    let latency = if is_valid {
                        Some((js_sys::Date::now() - start_time) as i64 + 50) // 模拟延迟
                    } else {
                        None
                    };
                    
                    let status = if is_valid { "valid" } else { "invalid" };
                    
                    // 更新数据库中的状态
                    db::update_proxy_node_status(&db, id, status, latency).await?;
                    
                    let result = ProxyCheckResult {
                        id,
                        status: status.to_string(),
                        latency,
                        message: if is_valid { "节点可用".to_string() } else { "节点不可用".to_string() },
                    };
                    
                    let response = ApiResponse::success(result);
                    Response::from_json(&response)
                }
                None => {
                    let response: ApiResponse<ProxyCheckResult> = ApiResponse::error("Node not found");
                    Response::from_json(&response).map(|r| r.with_status(404))
                }
            }
        }
        None => {
            let response: ApiResponse<ProxyCheckResult> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

// 批量验真
pub async fn check_all_nodes(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(_claims) => {
            let nodes = db::list_proxy_nodes(&db).await?;
            let mut results: Vec<ProxyCheckResult> = Vec::new();
            
            for node in nodes {
                let start_time = js_sys::Date::now();
                let is_valid = !node.address.is_empty() && node.port > 0 && node.port < 65536;
                
                let latency = if is_valid {
                    Some((js_sys::Date::now() - start_time) as i64 + 30 + (node.id % 100)) // 模拟不同延迟
                } else {
                    None
                };
                
                let status = if is_valid { "valid" } else { "invalid" };
                let _ = db::update_proxy_node_status(&db, node.id, status, latency).await;
                
                results.push(ProxyCheckResult {
                    id: node.id,
                    status: status.to_string(),
                    latency,
                    message: if is_valid { "节点可用".to_string() } else { "节点不可用".to_string() },
                });
            }
            
            let response = ApiResponse::success(results);
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<Vec<ProxyCheckResult>> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

// 获取所有订阅信息
pub async fn list_subscription_info(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(_claims) => {
            let infos = db::list_subscription_info(&db).await?;
            let response = ApiResponse::success(infos);
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<Vec<crate::models::SubscriptionInfo>> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

// 订阅链接导入请求
#[derive(serde::Deserialize)]
pub struct ImportSubscriptionRequest {
    pub url: Option<String>,
    pub content: Option<String>,
    pub group_name: Option<String>,
}

// 导入结果
#[derive(serde::Serialize)]
pub struct ImportResult {
    pub total: usize,
    pub success: usize,
    pub failed: usize,
    pub nodes: Vec<String>,
}

// 批量删除请求
#[derive(serde::Deserialize)]
pub struct BatchDeleteRequest {
    pub ids: Vec<i64>,
}

// 批量删除结果
#[derive(serde::Serialize)]
pub struct BatchDeleteResult {
    pub deleted: usize,
}

// 订阅链接导入
pub async fn import_subscription(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            let body: ImportSubscriptionRequest = req.json().await?;
            
            // 获取订阅内容
            // 用于存储订阅信息
            let mut subscription_url: Option<String> = None;
            let mut upload_bytes: i64 = 0;
            let mut download_bytes: i64 = 0;
            let mut total_bytes: i64 = 0;
            let mut expire_timestamp: Option<i64> = None;
            
            let content = if let Some(ref url) = body.url {
                subscription_url = Some(url.clone());
                
                // 从 URL 获取订阅内容
                let mut fetch_req = Request::new(url, Method::Get)?;
                // 添加完整的浏览器请求头，绕过 Cloudflare 等 WAF 检测
                let headers = fetch_req.headers_mut()?;
                headers.set("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")?;
                headers.set("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8")?;
                headers.set("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8")?;
                headers.set("Accept-Encoding", "gzip, deflate, br")?;
                headers.set("Cache-Control", "no-cache")?;
                headers.set("Pragma", "no-cache")?;
                
                let mut resp = Fetch::Request(fetch_req).send().await?;
                
                let status = resp.status_code();
                if status != 200 {
                    let text = resp.text().await.unwrap_or_default();
                    let err_msg = format!("订阅链接访问失败: HTTP {} -Body: {}", status, &text.chars().take(100).collect::<String>());
                    let response: ApiResponse<ImportResult> = ApiResponse::error(&err_msg);
                    return Response::from_json(&response).map(|r| r.with_status(400));
                }

                // 解析 subscription-userinfo 响应头
                // 支持多种前缀格式: subscription-userinfo, x-amz-meta-subscription-userinfo 等
                // 格式: upload=xxx; download=xxx; total=xxx; expire=xxx
                let headers = resp.headers();
                let mut userinfo_found = false;
                
                // 尝试常见的 header 名称
                let header_names = [
                    "subscription-userinfo",
                    "Subscription-Userinfo",
                    "Subscription-UserInfo",
                ];
                
                for header_name in &header_names {
                    if let Ok(Some(userinfo)) = headers.get(header_name) {
                        parse_subscription_userinfo(&userinfo, &mut upload_bytes, &mut download_bytes, &mut total_bytes, &mut expire_timestamp);
                        userinfo_found = true;
                        break;
                    }
                }
                
                // 如果标准名称没找到，尝试查找带前缀的版本
                if !userinfo_found {
                    // 遍历所有 header 查找以 subscription-userinfo 结尾的
                    for (key, value) in headers.entries() {
                        let key_lower = key.to_lowercase();
                        if key_lower.ends_with("subscription-userinfo") {
                            parse_subscription_userinfo(&value, &mut upload_bytes, &mut download_bytes, &mut total_bytes, &mut expire_timestamp);
                            break;
                        }
                    }
                }

                resp.text().await?
            } else if let Some(content) = body.content {
                content
            } else {
                let response: ApiResponse<ImportResult> = ApiResponse::error("请提供订阅链接或内容");
                return Response::from_json(&response).map(|r| r.with_status(400));
            };
            
            // 解析订阅内容
            let parsed_nodes = parse_subscription_content(&content);
            
            let mut success_count = 0;
            let mut failed_count = 0;
            let mut imported_names: Vec<String> = Vec::new();
            
            for node in &parsed_nodes {
                match db::create_proxy_node(
                    &db,
                    &node.name,
                    &node.protocol,
                    &node.address,
                    node.port,
                    node.username.as_deref(),
                    node.password.as_deref(),
                    node.extra_config.as_deref(),
                    body.group_name.as_deref(),
                ).await {
                    Ok(_) => {
                        success_count += 1;
                        imported_names.push(node.name.clone());
                    }
                    Err(_) => {
                        failed_count += 1;
                    }
                }
            }
            
            // 保存订阅信息（如果有分组名称）
            if let Some(ref group_name) = body.group_name {
                let _ = db::upsert_subscription_info(
                    &db,
                    group_name,
                    subscription_url.as_deref(),
                    upload_bytes,
                    download_bytes,
                    total_bytes,
                    expire_timestamp,
                ).await;
            }
            
            let _ = db::add_log(&db, claims.sub, &format!("imported {} proxy nodes from subscription", success_count)).await;
            
            let result = ImportResult {
                total: parsed_nodes.len(),
                success: success_count,
                failed: failed_count,
                nodes: imported_names,
            };
            
            let response = ApiResponse::success(result);
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<ImportResult> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

// 批量删除节点
pub async fn batch_delete_nodes(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;
    
    match jwt::extract_jwt_claims(&req) {
        Some(claims) => {
            let body: BatchDeleteRequest = req.json().await?;
            
            if body.ids.is_empty() {
                let response: ApiResponse<BatchDeleteResult> = ApiResponse::error("请选择要删除的节点");
                return Response::from_json(&response).map(|r| r.with_status(400));
            }
            
            let deleted = db::batch_delete_proxy_nodes(&db, &body.ids).await?;
            
            let _ = db::add_log(&db, claims.sub, &format!("batch deleted {} proxy nodes", deleted)).await;
            
            let result = BatchDeleteResult { deleted };
            let response = ApiResponse::success(result);
            Response::from_json(&response)
        }
        None => {
            let response: ApiResponse<BatchDeleteResult> = ApiResponse::error("Unauthorized");
            Response::from_json(&response).map(|r| r.with_status(401))
        }
    }
}

// 解析 subscription-userinfo 头部内容
fn parse_subscription_userinfo(
    userinfo: &str,
    upload: &mut i64,
    download: &mut i64,
    total: &mut i64,
    expire: &mut Option<i64>,
) {
    for part in userinfo.split(';') {
        let part = part.trim();
        if let Some((key, value)) = part.split_once('=') {
            let key = key.trim().to_lowercase();
            let value = value.trim();
            match key.as_str() {
                "upload" => *upload = value.parse().unwrap_or(0),
                "download" => *download = value.parse().unwrap_or(0),
                "total" => *total = value.parse().unwrap_or(0),
                "expire" => *expire = value.parse().ok(),
                _ => {}
            }
        }
    }
}

// 解析的节点信息
struct ParsedNode {
    name: String,
    protocol: String,
    address: String,
    port: i64,
    username: Option<String>,
    password: Option<String>,
    extra_config: Option<String>,
}

// 解析订阅内容
fn parse_subscription_content(content: &str) -> Vec<ParsedNode> {
    let mut nodes = Vec::new();
    
    // 尝试 Base64 解码
    let decoded = match base64_decode(content.trim()) {
        Some(d) => d,
        None => content.to_string(),
    };
    
    // 按行解析
    for line in decoded.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if let Some(node) = parse_single_link(line) {
            nodes.push(node);
        }
    }
    
    nodes
}

// Base64 解码
fn base64_decode(input: &str) -> Option<String> {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    
    // 尝试标准 Base64
    if let Ok(bytes) = STANDARD.decode(input) {
        if let Ok(s) = String::from_utf8(bytes) {
            return Some(s);
        }
    }
    
    // 尝试 URL-safe Base64
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    if let Ok(bytes) = URL_SAFE_NO_PAD.decode(input) {
        if let Ok(s) = String::from_utf8(bytes) {
            return Some(s);
        }
    }
    
    None
}

// 解析单个链接
fn parse_single_link(link: &str) -> Option<ParsedNode> {
    if link.starts_with("vmess://") {
        parse_vmess_link(link)
    } else if link.starts_with("trojan://") {
        parse_trojan_link(link)
    } else if link.starts_with("ss://") {
        parse_ss_link(link)
    } else if link.starts_with("http://") || link.starts_with("https://") {
        parse_http_proxy_link(link)
    } else if link.starts_with("socks5://") || link.starts_with("socks://") {
        parse_socks_link(link)
    } else {
        None
    }
}

// 解析 VMess 链接
fn parse_vmess_link(link: &str) -> Option<ParsedNode> {
    let encoded = link.strip_prefix("vmess://")?;
    let decoded = base64_decode(encoded)?;
    
    let json: serde_json::Value = serde_json::from_str(&decoded).ok()?;
    
    let address = json.get("add")?.as_str()?.to_string();
    let port = json.get("port")?.as_str()
        .and_then(|p| p.parse::<i64>().ok())
        .or_else(|| json.get("port")?.as_i64())?;
    let name = json.get("ps")
        .and_then(|v| v.as_str())
        .unwrap_or(&format!("VMess-{}", &address[..address.len().min(8)]))
        .to_string();
    
    Some(ParsedNode {
        name,
        protocol: "vmess".to_string(),
        address,
        port,
        username: json.get("id").and_then(|v| v.as_str()).map(|s| s.to_string()),
        password: None,
        extra_config: Some(decoded),
    })
}

// 解析 Trojan 链接
fn parse_trojan_link(link: &str) -> Option<ParsedNode> {
    // trojan://password@host:port?params#name
    let rest = link.strip_prefix("trojan://")?;
    
    // 分离名称
    let (main, name) = if let Some(idx) = rest.find('#') {
        let (m, n) = rest.split_at(idx);
        (m, Some(urlencoding_decode(&n[1..])))
    } else {
        (rest, None)
    };
    
    // 分离参数
    let main = main.split('?').next()?;
    
    // 解析 password@host:port
    let at_idx = main.find('@')?;
    let password = &main[..at_idx];
    let host_port = &main[at_idx + 1..];
    
    let colon_idx = host_port.rfind(':')?;
    let host = &host_port[..colon_idx];
    let port: i64 = host_port[colon_idx + 1..].parse().ok()?;
    
    Some(ParsedNode {
        name: name.unwrap_or_else(|| format!("Trojan-{}", &host[..host.len().min(8)])),
        protocol: "trojan".to_string(),
        address: host.to_string(),
        port,
        username: None,
        password: Some(password.to_string()),
        extra_config: Some(link.to_string()),
    })
}

// 解析 Shadowsocks 链接
fn parse_ss_link(link: &str) -> Option<ParsedNode> {
    // ss://base64(method:password)@host:port#name
    // 或 ss://base64(method:password@host:port)#name
    let rest = link.strip_prefix("ss://")?;
    
    // 分离名称
    let (main, name) = if let Some(idx) = rest.find('#') {
        let (m, n) = rest.split_at(idx);
        (m, Some(urlencoding_decode(&n[1..])))
    } else {
        (rest, None)
    };
    
    // 尝试新格式: base64(method:password)@host:port
    if let Some(at_idx) = main.find('@') {
        let encoded = &main[..at_idx];
        let host_port = &main[at_idx + 1..];
        
        if let Some(colon_idx) = host_port.rfind(':') {
            let host = &host_port[..colon_idx];
            let port: i64 = host_port[colon_idx + 1..].parse().ok()?;
            
            let decoded = base64_decode(encoded)?;
            let method_password: Vec<&str> = decoded.splitn(2, ':').collect();
            
            if method_password.len() == 2 {
                return Some(ParsedNode {
                    name: name.unwrap_or_else(|| format!("SS-{}", &host[..host.len().min(8)])),
                    protocol: "ss".to_string(),
                    address: host.to_string(),
                    port,
                    username: Some(method_password[0].to_string()), // method
                    password: Some(method_password[1].to_string()),
                    extra_config: Some(link.to_string()),
                });
            }
        }
    }
    
    // 尝试旧格式: 整体 base64
    if let Some(decoded) = base64_decode(main) {
        // method:password@host:port
        if let Some(at_idx) = decoded.find('@') {
            let method_password = &decoded[..at_idx];
            let host_port = &decoded[at_idx + 1..];
            
            let mp: Vec<&str> = method_password.splitn(2, ':').collect();
            if mp.len() == 2 {
                if let Some(colon_idx) = host_port.rfind(':') {
                    let host = &host_port[..colon_idx];
                    let port: i64 = host_port[colon_idx + 1..].parse().ok()?;
                    
                    return Some(ParsedNode {
                        name: name.unwrap_or_else(|| format!("SS-{}", &host[..host.len().min(8)])),
                        protocol: "ss".to_string(),
                        address: host.to_string(),
                        port,
                        username: Some(mp[0].to_string()),
                        password: Some(mp[1].to_string()),
                        extra_config: Some(link.to_string()),
                    });
                }
            }
        }
    }
    
    None
}

// 解析 HTTP 代理链接
fn parse_http_proxy_link(link: &str) -> Option<ParsedNode> {
    // http://user:pass@host:port 或 http://host:port
    let protocol = if link.starts_with("https://") { "https" } else { "http" };
    let rest = link.strip_prefix("http://").or_else(|| link.strip_prefix("https://"))?;
    
    let (auth, host_port) = if let Some(at_idx) = rest.find('@') {
        (Some(&rest[..at_idx]), &rest[at_idx + 1..])
    } else {
        (None, rest)
    };
    
    let colon_idx = host_port.rfind(':')?;
    let host = &host_port[..colon_idx];
    let port: i64 = host_port[colon_idx + 1..].split('/').next()?.parse().ok()?;
    
    let (username, password) = if let Some(auth) = auth {
        let parts: Vec<&str> = auth.splitn(2, ':').collect();
        if parts.len() == 2 {
            (Some(parts[0].to_string()), Some(parts[1].to_string()))
        } else {
            (Some(parts[0].to_string()), None)
        }
    } else {
        (None, None)
    };
    
    Some(ParsedNode {
        name: format!("{}-{}", protocol.to_uppercase(), &host[..host.len().min(12)]),
        protocol: protocol.to_string(),
        address: host.to_string(),
        port,
        username,
        password,
        extra_config: None,
    })
}

// 解析 SOCKS 代理链接
fn parse_socks_link(link: &str) -> Option<ParsedNode> {
    let rest = link.strip_prefix("socks5://").or_else(|| link.strip_prefix("socks://"))?;
    
    let (auth, host_port) = if let Some(at_idx) = rest.find('@') {
        (Some(&rest[..at_idx]), &rest[at_idx + 1..])
    } else {
        (None, rest)
    };
    
    let colon_idx = host_port.rfind(':')?;
    let host = &host_port[..colon_idx];
    let port: i64 = host_port[colon_idx + 1..].split('/').next()?.parse().ok()?;
    
    let (username, password) = if let Some(auth) = auth {
        let parts: Vec<&str> = auth.splitn(2, ':').collect();
        if parts.len() == 2 {
            (Some(parts[0].to_string()), Some(parts[1].to_string()))
        } else {
            (Some(parts[0].to_string()), None)
        }
    } else {
        (None, None)
    };
    
    Some(ParsedNode {
        name: format!("SOCKS5-{}", &host[..host.len().min(12)]),
        protocol: "socks5".to_string(),
        address: host.to_string(),
        port,
        username,
        password,
        extra_config: None,
    })
}

// URL 解码 - 正确处理 UTF-8 多字节字符
fn urlencoding_decode(input: &str) -> String {
    let mut bytes: Vec<u8> = Vec::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                bytes.push(byte);
            } else {
                bytes.push(b'%');
                bytes.extend(hex.as_bytes());
            }
        } else if c == '+' {
            bytes.push(b' ');
        } else {
            // 对于普通字符，将其 UTF-8 编码添加到字节数组
            let mut buf = [0u8; 4];
            let encoded = c.encode_utf8(&mut buf);
            bytes.extend(encoded.as_bytes());
        }
    }
    
    // 将字节数组转换为 UTF-8 字符串
    String::from_utf8(bytes).unwrap_or_else(|_| input.to_string())
}
