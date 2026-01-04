-- 添加分组字段到代理节点表
ALTER TABLE proxy_nodes ADD COLUMN group_name TEXT DEFAULT '';

-- 创建分组索引
CREATE INDEX IF NOT EXISTS idx_proxy_nodes_group ON proxy_nodes(group_name);
