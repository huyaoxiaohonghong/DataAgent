-- 订阅信息表，按分组存储订阅元数据
CREATE TABLE IF NOT EXISTS subscription_info (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_name TEXT NOT NULL UNIQUE,
    subscription_url TEXT,
    upload_bytes INTEGER DEFAULT 0,
    download_bytes INTEGER DEFAULT 0,
    total_bytes INTEGER DEFAULT 0,
    expire_timestamp INTEGER,
    last_update_at TEXT DEFAULT (datetime('now')),
    created_at TEXT DEFAULT (datetime('now'))
);

-- 添加索引
CREATE INDEX IF NOT EXISTS idx_subscription_group_name ON subscription_info(group_name);
