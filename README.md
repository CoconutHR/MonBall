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

---

## 使用指南

这是一份面向零基础用户的 MonBall 使用说明，帮助你快速上手这个轻量系统监控悬浮窗工具。

### 这是什么？

MonBall 是一个桌面小工具，它会在你的 Windows 桌面上显示一个半透明的小气泡，实时展示远程 Linux 服务器的运行状态（CPU、内存、磁盘、网络速度）。适合需要随时关注服务器健康状况的开发者、运维人员。

### 整体架构（两部分）

MonBall 由两个程序组成，它们配合工作：

```
┌─────────────────┐         HTTP 请求          ┌─────────────────┐
│  Windows 桌面   │  ◄───────────────────────►  │  Linux 服务器   │
│  (Client 悬浮窗) │      每秒拉取一次数据       │  (Server Agent)  │
└─────────────────┘                             └─────────────────┘
```

- **Server Agent**：运行在你的 Linux 服务器上，负责采集系统数据
- **Client 悬浮窗**：运行在你的 Windows 电脑上，负责展示数据

### 默认配置

| 配置项 | 默认值 | 说明 |
|--------|--------|------|
| 端口 | `26666` | Agent 监听端口 |
| Token | `monball` | 鉴权令牌 |
| 安装路径 | `/opt/monball-agent/` | Linux 服务端安装目录 |

### 第一步：部署 Server Agent（Linux 服务器）

#### 方式一：一键安装（推荐）

SSH 登录到你的服务器，执行：

```bash
curl -sSL https://raw.githubusercontent.com/CoconutHR/MonBall/main/server/install.sh | sudo bash
```

安装完成后，Agent 会自动启动并注册为系统服务（开机自启）。

如需自定义端口和 Token：

```bash
curl -sSL https://raw.githubusercontent.com/CoconutHR/MonBall/main/server/install.sh | MONBALL_PORT=9999 MONBALL_TOKEN=your-secret sudo -E bash
```

#### 方式二：手动安装

##### 下载

到 [GitHub Releases](https://github.com/CoconutHR/MonBall/releases) 页面下载最新的 `sysmon-agent` 文件。

##### 上传并安装

```bash
# 上传到服务器
scp sysmon-agent 你的用户名@你的服务器IP:/tmp/

# SSH 登录服务器
ssh 你的用户名@你的服务器IP

# 安装到 /opt/monball-agent/
sudo mkdir -p /opt/monball-agent
sudo mv /tmp/sysmon-agent /opt/monball-agent/
sudo chmod +x /opt/monball-agent/sysmon-agent
```

##### 注册系统服务

```bash
sudo tee /etc/systemd/system/monball-agent.service << 'EOF'
[Unit]
Description=MonBall System Monitor Agent
After=network.target

[Service]
Type=simple
Environment="MONITOR_TOKEN=monball"
Environment="PORT=26666"
ExecStart=/opt/monball-agent/sysmon-agent
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable monball-agent
sudo systemctl start monball-agent
```

#### 验证是否启动成功

```bash
# 健康检查（无需 Token）
curl http://localhost:26666/health

# 数据接口（需要 Token）
curl -H "x-monitor-token: monball" http://localhost:26666/api/v1/stats
```

健康检查返回：

```json
{"status":"ok","service":"monball-agent","version":"0.1.2"}
```

数据接口返回：

```json
{"cpu_usage":12.5,"mem_usage":45.2,"disk_usage":67.8,"net_rx_rate":1024,"net_tx_rate":512,"cpu_temp":45.0,"timestamp":1717670400}
```

#### 防火墙放行

确保服务器的 26666 端口对你的电脑开放：

```bash
# 如果用的 ufw
sudo ufw allow 26666/tcp

# 如果用的 firewalld
sudo firewall-cmd --add-port=26666/tcp --permanent
sudo firewall-cmd --reload
```

### 第二步：安装 Client 悬浮窗（Windows 电脑）

#### 获取安装包

到 [GitHub Releases](https://github.com/CoconutHR/MonBall/releases) 页面下载最新的 `.exe`（推荐）或 `.msi` 安装包。

#### 安装

双击下载的安装包，按照提示完成安装即可。

#### 首次启动

安装完成后启动 MonBall，你会在桌面看到一个半透明的小气泡。如果 Server Agent 已经在运行且网络通畅，默认就能看到数据（因为客户端默认端口和 Token 与服务端一致）。

如果显示"连接失败"，只需要在配置中把 IP 改为你的服务器地址即可。

### 第三步：配置连接

#### 打开配置窗口

在系统托盘（屏幕右下角）找到 MonBall 的图标，**右键点击**，选择"配置"。

#### 填写服务器信息

在配置窗口中填写：

| 字段 | 默认值 | 说明 |
|------|--------|------|
| IP 地址 | `127.0.0.1` | 改为你的 Linux 服务器 IP |
| 端口 | `26666` | 与 Agent 端口一致 |
| Token | `monball` | 与 Agent Token 一致 |

#### 选择显示指标

勾选你想在悬浮窗中看到的指标：

- **CPU** — 处理器使用率
- **MEM** — 内存使用率
- **DISK** — 磁盘使用率
- **NET** — 网络上传/下载速度
- **TEMP** — CPU 温度

#### 保存

点击"保存"按钮，配置窗口会自动关闭，悬浮窗将开始显示实时数据。

### 日常使用

#### 托盘菜单功能

右键点击系统托盘中的 MonBall 图标：

| 菜单项 | 功能 |
|--------|------|
| 锁定 (穿透) | 悬浮窗变为完全穿透状态，鼠标点击会穿过它到达下面的窗口 |
| 解锁 (可拖动) | 悬浮窗可以用鼠标拖动到任意位置 |
| 更透明 | 降低背景不透明度，使悬浮窗更透明（仅影响背景，不影响文字） |
| 更不透明 | 增加背景不透明度，使悬浮窗更不透明 |
| 配置 | 打开配置窗口修改设置 |
| 退出 | 关闭 MonBall |

#### 典型操作流程

1. 刚启动时，悬浮窗默认**可拖动**，把它拖到你喜欢的位置
2. 位置确定后，在托盘菜单点击**"锁定"**，这样鼠标就会穿透悬浮窗，不影响正常操作
3. 想移动位置时，再点**"解锁"**即可

#### 数据含义

```
CPU   23.5%        ← 服务器 CPU 使用率
MEM   61.2%        ← 服务器内存使用率
DISK  45.0%        ← 服务器磁盘使用率（根分区）
TEMP  52°C         ← 服务器 CPU 温度
↓1.2 K/s ↑0.5 K/s ← 网络下载/上传速度
```

### 常见问题

#### 悬浮窗一直显示"连接失败"

可能的原因：

1. **Server Agent 没有启动** — 登录服务器运行 `systemctl status monball-agent` 检查
2. **IP 地址未修改** — 默认是 `127.0.0.1`（本机），需要改为服务器真实 IP
3. **防火墙拦截** — 确认服务器 26666 端口已放行
4. **网络不通** — 在 Windows 命令行中 `ping 你的服务器IP` 看看能否连通

#### 悬浮窗不见了

可能被拖到屏幕外面了。在托盘菜单点"退出"，然后重新启动 MonBall，它会回到默认位置。

#### 想开机自启动

将 MonBall 的快捷方式放入 Windows 启动文件夹：

1. 按 `Win + R`，输入 `shell:startup`，回车
2. 把 MonBall 的快捷方式复制到打开的文件夹中

#### Agent 管理命令

```bash
# 查看状态
systemctl status monball-agent

# 查看实时日志
journalctl -u monball-agent -f

# 重启
systemctl restart monball-agent

# 停止
systemctl stop monball-agent
```

#### 卸载 Agent

```bash
sudo systemctl stop monball-agent
sudo systemctl disable monball-agent
sudo rm -f /etc/systemd/system/monball-agent.service
sudo systemctl daemon-reload
sudo rm -rf /opt/monball-agent
```

#### 升级 Agent

重新运行一键安装脚本即可，会自动覆盖旧版本并重启服务：

```bash
curl -sSL https://raw.githubusercontent.com/CoconutHR/MonBall/main/server/install.sh | sudo bash
```

### 安全建议

1. **修改默认 Token**：生产环境建议修改默认的 `monball` 为更复杂的字符串
2. **限制访问 IP**：如果可能，在防火墙中只允许你的电脑 IP 访问 Agent 端口
3. **使用非默认端口**：如有安全顾虑，可将端口改为其他数字

修改方法：编辑 systemd 服务文件中的 `Environment` 行，然后 `systemctl daemon-reload && systemctl restart monball-agent`。

### 技术支持

如有问题或建议，欢迎到 [GitHub Issues](https://github.com/CoconutHR/MonBall/issues) 提交反馈。

## License

MIT
