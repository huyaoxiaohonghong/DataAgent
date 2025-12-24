# Edge Management System

全栈边缘管理系统 - 完全运行在 Cloudflare 边缘网络上

## 架构

- **Frontend**: Vue 3 + Ant Design Vue (Cloudflare Pages)
- **Gateway/API**: Cloudflare Worker (Rust)
- **Cache/Session**: Cloudflare KV
- **Database**: Cloudflare D1 (SQLite)

## 项目结构

```
├── backend/                 # Cloudflare Worker (Rust)
│   ├── src/
│   │   ├── lib.rs          # 入口与路由
│   │   ├── auth.rs         # Session 管理
│   │   ├── db.rs           # D1 数据库操作
│   │   ├── models.rs       # 数据模型
│   │   ├── utils.rs        # 工具函数
│   │   └── handlers/       # API 处理器
│   ├── migrations/         # D1 迁移文件
│   ├── Cargo.toml
│   └── wrangler.toml
└── frontend/               # Vue 3 + Ant Design Vue
    ├── src/
    │   ├── api/            # API 客户端
    │   ├── layouts/        # 布局组件
    │   ├── router/         # 路由配置
    │   ├── stores/         # Pinia 状态管理
    │   ├── styles/         # 全局样式
    │   ├── types/          # TypeScript 类型
    │   └── views/          # 页面组件
    ├── package.json
    └── vite.config.ts
```

## 开发环境设置

### 前置要求

- Node.js 18+
- Rust (rustup)
- Wrangler CLI (`npm install -g wrangler`)

### 后端设置

1. 创建 D1 数据库:
```bash
cd backend
wrangler d1 create edge-management-db
```

2. 更新 `wrangler.toml` 中的 `database_id`

3. 创建 KV 命名空间:
```bash
wrangler kv:namespace create SESSION_KV
```

4. 更新 `wrangler.toml` 中的 KV `id`

5. 运行数据库迁移:
```bash
wrangler d1 execute edge-management-db --file=./migrations/0001_initial.sql
```

6. 启动开发服务器:
```bash
wrangler dev
```

### 前端设置

1. 安装依赖:
```bash
cd frontend
npm install
```

2. 启动开发服务器:
```bash
npm run dev
```

3. 访问 http://localhost:5173

### 默认账号

- 用户名: `admin`
- 密码: `admin123`

## 部署

### 部署后端 (Cloudflare Worker)

```bash
cd backend
wrangler publish
```

### 部署前端 (Cloudflare Pages)

```bash
cd frontend
npm run build
wrangler pages publish dist
```

## API 端点

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | /api/auth/login | 用户登录 |
| POST | /api/auth/logout | 用户登出 |
| GET | /api/auth/check | 检查会话 |
| GET | /api/user/profile | 获取当前用户信息 |
| GET | /api/users | 获取用户列表 |
| POST | /api/users | 创建用户 |
| GET | /api/logs | 获取操作日志 |

## 技术特点

- **零源服务器**: 完全运行于 Cloudflare 边缘网络
- **低延迟**: 全球分布式节点，就近访问
- **Rust 性能**: 后端使用 Rust 编写，高效安全
- **现代化前端**: Vue 3 + TypeScript + Ant Design Vue
- **玻璃拟态设计**: 美观的暗色主题 UI
