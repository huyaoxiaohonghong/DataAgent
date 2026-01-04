-- 代理节点配置表
CREATE TABLE IF NOT EXISTS proxy_nodes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    protocol TEXT NOT NULL,          -- http, https, socks5, vmess, trojan 等
    address TEXT NOT NULL,           -- 服务器地址
    port INTEGER NOT NULL,           -- 端口
    username TEXT,                   -- 用户名（可选）
    password TEXT,                   -- 密码（可选）
    extra_config TEXT,               -- 额外配置（JSON格式，存放其他参数）
    status TEXT DEFAULT 'unknown',   -- unknown, valid, invalid
    latency INTEGER,                 -- 延迟（毫秒）
    last_check_at TEXT,              -- 上次检测时间
    created_at TEXT DEFAULT (datetime('now')),
    updated_at TEXT DEFAULT (datetime('now'))
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_proxy_nodes_status ON proxy_nodes(status);
CREATE INDEX IF NOT EXISTS idx_proxy_nodes_protocol ON proxy_nodes(protocol);
