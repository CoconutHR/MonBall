# MonBall 使用指南

这是一份面向零基础用户的 MonBall 使用说明，帮助你快速上手这个轻量系统监控悬浮窗工具。

## 这是什么？

MonBall 是一个桌面小工具，它会在你的 Windows 桌面上显示一个半透明的小气泡，实时展示远程 Linux 服务器的运行状态（CPU、内存、磁盘、网络速度）。适合需要随时关注服务器健康状况的开发者、运维人员。

## 整体架构（两部分）

MonBall 由两个程序组成，它们配合工作：

```
┌─────────────────┐         HTTP 请求          ┌─────────────────┐
│  Windows 桌面   │  ◄───────────────────────►  │  Linux 服务器   │
│  (Client 悬浮窗) │      每秒拉取一次数据       │  (Server Agent)  │
└─────────────────┘                             └─────────────────┘
```

- **Server Agent**：运行在你的 Linux 服务器上，负责采集系统数据
- **Client 悬浮窗**：运行在你的 Windows 电脑上，负责展示数据

## 第一步：部署 Server Agent（Linux 服务器）

### 获取程序

到 [GitHub Releases](https://github.com/CoconutHR/MonBall/releases) 页面下载最新的 `sysmon-agent` 文件。

### 上传到服务器

```bash
# 用 scp 上传到你的服务器（替换为你的服务器信息）
scp sysmon-agent 你的用户名@你的服务器IP:/home/你的用户名/
```

### 启动 Agent

```bash
# SSH 登录到服务器
ssh 你的用户名@你的服务器IP

# 给予执行权限
chmod +x sysmon-agent

# 设置 Token（用于安全验证，可以改成你喜欢的密码）
export MONITOR_TOKEN="my-secret-token-123"

# 设置端口（默认 8080，可选修改）
export PORT=8080

# 后台启动
nohup ./sysmon-agent &
```

### 验证是否启动成功

```bash
# 在服务器上测试
curl -H "x-monitor-token: my-secret-token-123" http://localhost:8080/api/v1/stats
```

如果看到类似下面的 JSON 输出，说明启动成功：

```json
{"cpu_usage":12.5,"mem_usage":45.2,"disk_usage":67.8,"net_rx_rate":1024,"net_tx_rate":512,"timestamp":1717670400}
```

### 防火墙放行

确保服务器的 8080 端口（或你设置的端口）对你的电脑开放：

```bash
# 如果用的 ufw
sudo ufw allow 8080/tcp

# 如果用的 firewalld
sudo firewall-cmd --add-port=8080/tcp --permanent
sudo firewall-cmd --reload
```

## 第二步：安装 Client 悬浮窗（Windows 电脑）

### 获取安装包

到 [GitHub Releases](https://github.com/CoconutHR/MonBall/releases) 页面下载最新的 `.msi` 或 `.exe` 安装包。

### 安装

双击下载的安装包，按照提示完成安装即可。

### 首次启动

安装完成后启动 MonBall，你会在桌面右上角看到一个半透明的小气泡（初始显示"连接失败"是正常的，因为还没配置服务器地址）。

## 第三步：配置连接

### 打开配置窗口

在系统托盘（屏幕右下角）找到 MonBall 的图标（蓝色圆形），**右键点击**，选择"配置"。

### 填写服务器信息

在配置窗口中填写：

| 字段 | 说明 | 示例 |
|------|------|------|
| IP 地址 | 你的 Linux 服务器 IP | `192.168.1.100` |
| 端口 | Agent 监听的端口 | `8080` |
| Token | 你在服务器设置的 MONITOR_TOKEN | `my-secret-token-123` |

### 选择显示指标

勾选你想在悬浮窗中看到的指标：

- **CPU** — 处理器使用率
- **MEM** — 内存使用率
- **DISK** — 磁盘使用率
- **NET** — 网络上传/下载速度

### 保存

点击"保存"按钮，配置窗口会自动关闭，悬浮窗将开始显示实时数据。

## 日常使用

### 托盘菜单功能

右键点击系统托盘中的 MonBall 图标：

| 菜单项 | 功能 |
|--------|------|
| 锁定 (穿透) | 悬浮窗变为完全穿透状态，鼠标点击会穿过它到达下面的窗口 |
| 解锁 (可拖动) | 悬浮窗可以用鼠标拖动到任意位置 |
| 配置 | 打开配置窗口修改设置 |
| 退出 | 关闭 MonBall |

### 典型操作流程

1. 刚启动时，悬浮窗默认**可拖动**，把它拖到你喜欢的位置
2. 位置确定后，在托盘菜单点击**"锁定"**，这样鼠标就会穿透悬浮窗，不影响正常操作
3. 想移动位置时，再点**"解锁"**即可

### 数据含义

```
CPU   23.5%        ← 服务器 CPU 使用率
MEM   61.2%        ← 服务器内存使用率
DISK  45.0%        ← 服务器磁盘使用率（根分区）
↓1.2 K/s ↑0.5 K/s ← 网络下载/上传速度
```

## 常见问题

### 悬浮窗一直显示"连接失败"

可能的原因：

1. **Server Agent 没有启动** — 登录服务器检查进程是否存在：`ps aux | grep sysmon-agent`
2. **IP/端口/Token 填写错误** — 打开配置检查一下
3. **防火墙拦截** — 确认服务器端口已放行
4. **网络不通** — 在 Windows 命令行中 ping 一下服务器 IP

### 悬浮窗不见了

可能被拖到屏幕外面了。在托盘菜单点"退出"，然后重新启动 MonBall，它会回到默认位置。

### 想开机自启动

将 MonBall 的快捷方式放入 Windows 启动文件夹：

1. 按 `Win + R`，输入 `shell:startup`，回车
2. 把 MonBall 的快捷方式复制到打开的文件夹中

### Agent 想设置为系统服务（开机自启）

创建 systemd 服务文件：

```bash
sudo tee /etc/systemd/system/sysmon-agent.service << 'EOF'
[Unit]
Description=MonBall System Monitor Agent
After=network.target

[Service]
Type=simple
Environment="MONITOR_TOKEN=my-secret-token-123"
Environment="PORT=8080"
ExecStart=/home/你的用户名/sysmon-agent
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# 启用并启动
sudo systemctl enable sysmon-agent
sudo systemctl start sysmon-agent

# 查看状态
sudo systemctl status sysmon-agent
```

## 安全建议

1. **设置强 Token**：不要使用默认的 `secret123`，换一个复杂的字符串
2. **限制访问 IP**：如果可能，在防火墙中只允许你的电脑 IP 访问 Agent 端口
3. **使用非默认端口**：把端口改为一个不常见的数字（如 `9527`），可以减少被扫描的风险

## 技术支持

如有问题或建议，欢迎到 [GitHub Issues](https://github.com/CoconutHR/MonBall/issues) 提交反馈。
