# MonBall 🎈

轻量系统监控悬浮窗，采用 Client-Server (Agent) 架构。

## 架构

- **Server (Agent)**：Rust 后台服务，部署于 Linux，高频采集 CPU / 内存 / 磁盘 / 网络指标，通过 HTTP API 暴露数据
- **Client (Tauri v2)**：Windows 透明悬浮窗 + 独立配置窗口，纯 HTML + 原生 JS，无前端框架

## 功能

- 实时监控 CPU、内存、磁盘、网络速率
- 透明悬浮窗常驻桌面，支持穿透/拖动切换
- 系统托盘菜单控制（锁定/解锁/配置/退出）
- 可自定义显示指标和服务器连接
- 配置持久化，重启保持

## 技术栈

- Rust + Axum + Tokio + sysinfo（Server）
- Tauri v2 + 纯 HTML/CSS/JS（Client）
- 零 Shell 依赖，异步无阻塞

## 构建

### Server

```bash
cd server
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### Client

```bash
cd client
npm install
npx tauri build
```

## License

MIT
