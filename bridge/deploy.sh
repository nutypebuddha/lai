#!/bin/bash
set -e

echo "=== CID Bridge Deploy ==="

# 1. Install deps
echo "[1/5] npm install"
cd "$(dirname "$0")"
npm install --production

# 2. Ensure CID binary exists
echo "[2/5] check CID binary"
CID=${CID_BINARY:-$(find /root/Laverna/target/release/lai-gate -maxdepth 0 2>/dev/null || true)}
if [ -z "$CID" ] || [ ! -f "$CID" ]; then
  echo "ERROR: lai-gate binary not found. Build first or set CID_BINARY env var."
  exit 1
fi

# 3. Install systemd service
echo "[3/5] install systemd service"
cp config/cid-bridge.service /etc/systemd/system/cid-bridge.service
sed -i "s|ExecStart=.*|ExecStart=/usr/bin/node $(pwd)/src/index.js|" /etc/systemd/system/cid-bridge.service
sed -i "s|WorkingDirectory=.*|WorkingDirectory=$(pwd)|" /etc/systemd/system/cid-bridge.service
sed -i "s|Environment=CID_BINARY=.*|Environment=CID_BINARY=$CID|" /etc/systemd/system/cid-bridge.service

# 4. Install nginx config
echo "[4/5] configure nginx"
cp config/nginx.conf /etc/nginx/sites-available/cid-bridge
ln -sf /etc/nginx/sites-available/cid-bridge /etc/nginx/sites-enabled/cid-bridge
rm -f /etc/nginx/sites-enabled/default
nginx -t
systemctl reload nginx || systemctl start nginx

# 5. Start service
echo "[5/5] start cid-bridge"
systemctl daemon-reload
systemctl enable cid-bridge
systemctl start cid-bridge

echo "=== Deploy OK ==="
echo "Health: http://localhost/health"
