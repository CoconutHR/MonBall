#!/bin/bash
# MonBall Agent 一键安装脚本
# 用法: curl -sSL https://raw.githubusercontent.com/CoconutHR/MonBall/main/server/install.sh | sudo bash
# 或:   sudo bash install.sh

set -e

# ===== 配置 =====
INSTALL_DIR="/opt/monball-agent"
SERVICE_NAME="monball-agent"
BINARY_NAME="sysmon-agent"
REPO="CoconutHR/MonBall"
PORT="${MONBALL_PORT:-26666}"
TOKEN="${MONBALL_TOKEN:-monball}"

# ===== 颜色输出 =====
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# ===== 检查权限 =====
if [ "$(id -u)" -ne 0 ]; then
    error "请使用 root 权限运行此脚本 (sudo bash install.sh)"
fi

# ===== 检测架构 =====
ARCH=$(uname -m)
if [ "$ARCH" != "x86_64" ]; then
    error "当前仅支持 x86_64 架构，检测到: $ARCH"
fi

# ===== 获取最新版本 =====
info "正在获取最新版本信息..."
LATEST_TAG=$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$LATEST_TAG" ]; then
    error "无法获取最新版本信息，请检查网络连接"
fi
info "最新版本: ${LATEST_TAG}"

# ===== 下载二进制文件 =====
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${LATEST_TAG}/${BINARY_NAME}"
info "正在下载 ${BINARY_NAME}..."
mkdir -p "${INSTALL_DIR}"
curl -sSL "${DOWNLOAD_URL}" -o "${INSTALL_DIR}/${BINARY_NAME}"
chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
info "已安装到 ${INSTALL_DIR}/${BINARY_NAME}"

# ===== 写入版本信息 =====
echo "${LATEST_TAG}" > "${INSTALL_DIR}/VERSION"

# ===== 创建 systemd 服务 =====
info "正在配置 systemd 服务..."
cat > /etc/systemd/system/${SERVICE_NAME}.service << EOF
[Unit]
Description=MonBall System Monitor Agent
After=network.target

[Service]
Type=simple
Environment="MONITOR_TOKEN=${TOKEN}"
Environment="PORT=${PORT}"
ExecStart=${INSTALL_DIR}/${BINARY_NAME}
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
EOF

# ===== 启动服务 =====
systemctl daemon-reload
systemctl enable ${SERVICE_NAME}
systemctl restart ${SERVICE_NAME}

# ===== 验证 =====
sleep 2
if systemctl is-active --quiet ${SERVICE_NAME}; then
    info "服务启动成功！"
else
    warn "服务可能未正常启动，请检查: journalctl -u ${SERVICE_NAME} -n 20"
fi

# ===== 打印摘要 =====
echo ""
echo "============================================"
echo "  MonBall Agent 安装完成!"
echo "============================================"
echo ""
echo "  版本:    ${LATEST_TAG}"
echo "  安装路径: ${INSTALL_DIR}/${BINARY_NAME}"
echo "  端口:    ${PORT}"
echo "  Token:   ${TOKEN}"
echo ""
echo "  健康检查: curl http://localhost:${PORT}/health"
echo "  数据接口: curl -H 'x-monitor-token: ${TOKEN}' http://localhost:${PORT}/api/v1/stats"
echo ""
echo "  管理命令:"
echo "    查看状态: systemctl status ${SERVICE_NAME}"
echo "    查看日志: journalctl -u ${SERVICE_NAME} -f"
echo "    重启服务: systemctl restart ${SERVICE_NAME}"
echo "    停止服务: systemctl stop ${SERVICE_NAME}"
echo "    卸载:    systemctl stop ${SERVICE_NAME} && systemctl disable ${SERVICE_NAME} && rm -f /etc/systemd/system/${SERVICE_NAME}.service && rm -rf ${INSTALL_DIR}"
echo ""
echo "  如需自定义端口和 Token，请设置环境变量后重新安装:"
echo "    MONBALL_PORT=9999 MONBALL_TOKEN=your-token sudo bash install.sh"
echo ""
