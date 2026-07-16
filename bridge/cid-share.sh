#!/bin/bash
# ============================================================================
# CID Bridge — Share Script (Dual Tunnel)
# ============================================================================
# Starts cid-bridge + Cloudflare Quick Tunnel + localhost.run tunnel + watchdog.
# The bridge serves a live dashboard showing both URLs.
#
# Usage:
#   ./cid-share.sh                          # starts with Cloudflare + localhost.run
#   NTFY_TOPIC=mytopic ./cid-share.sh       # with phone notifications
#
# Tunnels:
#   🌩️  Cloudflare Quick Tunnel (trycloudflare.com) — generic access
#   🚀  localhost.run (lhr.life) — ChatGPT-friendly domain
#
# URLs are saved to:
#   current-url.txt      (Cloudflare)
#   current-url-alt.txt  (localhost.run)
# ============================================================================

set -uo pipefail

BRIDGE_DIR="$(cd "$(dirname "$0")" && pwd)"
URL_FILE="$BRIDGE_DIR/current-url.txt"
URL_FILE_ALT="$BRIDGE_DIR/current-url-alt.txt"
TUNNEL_LOG="/tmp/cid-tunnel.log"
TUNNEL_ALT_LOG="/tmp/cid-tunnel-alt.log"
BRIDGE_LOG="/tmp/cid-bridge.log"

# ── ntfy.sh push notification ──────────────────────────────────
# 1. Install ntfy app on your phone
# 2. Subscribe to a topic (e.g. "cid-bridge-unicorn42")
# 3. Set it here or in your env:
#
#     export NTFY_TOPIC="cid-bridge-unicorn42"
#
# Then every time cid-share runs, your phone buzzes with both URLs.
NTFY_TOPIC="${NTFY_TOPIC:-gothgirlgarefield}"

notify_ntfy() {
  local url="$1"
  local url_alt="$2"
  [ -z "$NTFY_TOPIC" ] && return 0
  curl -s -o /dev/null \
    -H "Title: 🚀 CID Bridge Live" \
    -H "Tags: computer" \
    -H "Priority: default" \
    -H "Click: $url_alt/" \
    -d "CID Bridge is live!

🌩️  Cloudflare: $url/
🚀  ChatGPT:    $url_alt/

Dashboard: $url_alt/
Validate:  curl -X POST $url_alt/validate -d '{\"text\":\"2+2=4\",\"context\":\"math\"}'" \
    "https://ntfy.sh/$NTFY_TOPIC" &
  echo "  📱 ntfy notification sent"
}

GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo ""
echo "╔════════════════════════════════════════════════════════╗"
echo "║        🚀  CID Bridge — Share Tool                    ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

# --------------------------------------------------
# Kill old tunnel + bridge, start fresh
# --------------------------------------------------
echo -e "${YELLOW}[1/4]${NC} Starting cid-bridge..."
pkill -f "cid-bridge" 2>/dev/null || true
pkill -f "localhost.run" 2>/dev/null || true
pkill -f "cloudflared tunnel" 2>/dev/null || true
sleep 1
CID_BINARY=/root/Laverna/target/release/lai-gate PORT=3000 \
  nohup node "$BRIDGE_DIR/src/index.js" > "$BRIDGE_LOG" 2>&1 &
sleep 2
echo "  ✓ cid-bridge running on port 3000"
echo ""

# --------------------------------------------------
# Tunnel 1: Cloudflare Quick Tunnel
# --------------------------------------------------
echo -e "${YELLOW}[2/4]${NC} Starting Cloudflare Quick Tunnel..."
> "$TUNNEL_LOG"
setsid nohup cloudflared tunnel --url http://localhost:3000 --no-autoupdate > "$TUNNEL_LOG" 2>&1 &
TUNNEL_PID=$!

echo "  Waiting for Cloudflare..."
URL=""
for i in $(seq 1 30); do
  URL=$(grep -oE 'https://[a-z0-9-]+\.trycloudflare\.com' "$TUNNEL_LOG" 2>/dev/null | head -1)
  [ -n "$URL" ] && break
  sleep 1
done

if [ -z "$URL" ]; then
  echo -e "  ${YELLOW}⚠  Cloudflare: Could not get URL${NC}"
  tail -3 "$TUNNEL_LOG"
  URL="(unavailable)"
fi
echo "$URL" > "$URL_FILE"
echo "  ✓ Cloudflare: $URL"
echo ""

# --------------------------------------------------
# Tunnel 2: localhost.run (ChatGPT-friendly)
# --------------------------------------------------
echo -e "${YELLOW}[3/4]${NC} Starting localhost.run tunnel..."
> "$TUNNEL_ALT_LOG"
setsid nohup ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
  -o ServerAliveInterval=30 -o ServerAliveCountMax=3 \
  -R 80:localhost:3000 nokey@localhost.run \
  > "$TUNNEL_ALT_LOG" 2>&1 &
TUNNEL_ALT_PID=$!

echo "  Waiting for localhost.run..."
URL_ALT=""
for i in $(seq 1 30); do
  URL_ALT=$(grep -oE 'https://[a-z0-9]+\.lhr\.life' "$TUNNEL_ALT_LOG" 2>/dev/null | head -1)
  [ -n "$URL_ALT" ] && break
  sleep 1
done

if [ -z "$URL_ALT" ]; then
  echo -e "  ${YELLOW}⚠  localhost.run: Could not get URL${NC}"
  tail -3 "$TUNNEL_ALT_LOG"
  URL_ALT="(unavailable)"
fi
echo "$URL_ALT" > "$URL_FILE_ALT"
echo "  ✓ localhost.run: $URL_ALT"
echo ""

notify_ntfy "$URL" "$URL_ALT"
echo ""

# --------------------------------------------------
# Show the goods
# --------------------------------------------------
echo -e "${YELLOW}[4/4]${NC} Ready!"
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║${NC}                                                        ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}  🌩️  Cloudflare:    ${CYAN}${URL}${NC}  ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}  🚀  ChatGPT:      ${CYAN}${URL_ALT}${NC}  ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}                                                        ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}  Dashboard:  ${URL}/  |  ${URL_ALT}/  ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}  Health:     ${URL}/health         ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}  Status:     ${URL}/status          ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}                                                        ${GREEN}║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "  URLs saved to: $URL_FILE and $URL_FILE_ALT"
echo "  Bridge logs:    $BRIDGE_LOG"
echo "  Cloudflare log: $TUNNEL_LOG"
echo "  localhost.run:  $TUNNEL_ALT_LOG"
echo ""

# --------------------------------------------------
# Watchdog (monitor both tunnels)
# --------------------------------------------------
echo "  Watching tunnels (CF PID $TUNNEL_PID, LR PID $TUNNEL_ALT_PID)... Ctrl+C to stop."
echo ""

while true; do
  # Check Cloudflare
  if ! kill -0 $TUNNEL_PID 2>/dev/null; then
    echo ""
    echo -e "${YELLOW}⚠  Cloudflare tunnel died! Restarting...${NC}"
    > "$TUNNEL_LOG"
    setsid nohup cloudflared tunnel --url http://localhost:3000 --no-autoupdate > "$TUNNEL_LOG" 2>&1 &
    TUNNEL_PID=$!

    for i in $(seq 1 30); do
      URL=$(grep -oE 'https://[a-z0-9-]+\.trycloudflare\.com' "$TUNNEL_LOG" 2>/dev/null | head -1)
      if [ -n "$URL" ]; then
        echo "$URL" > "$URL_FILE"
        echo -e "${GREEN}  Cloudflare: $URL${NC}"
        break
      fi
      sleep 1
    done
  fi

  # Check localhost.run
  if ! kill -0 $TUNNEL_ALT_PID 2>/dev/null; then
    echo ""
    echo -e "${YELLOW}⚠  localhost.run tunnel died! Restarting...${NC}"
    > "$TUNNEL_ALT_LOG"
    setsid nohup ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
      -o ServerAliveInterval=30 -o ServerAliveCountMax=3 \
      -R 80:localhost:3000 nokey@localhost.run \
      > "$TUNNEL_ALT_LOG" 2>&1 &
    TUNNEL_ALT_PID=$!

    for i in $(seq 1 30); do
      URL_ALT=$(grep -oE 'https://[a-z0-9]+\.lhr\.life' "$TUNNEL_ALT_LOG" 2>/dev/null | head -1)
      if [ -n "$URL_ALT" ]; then
        echo "$URL_ALT" > "$URL_FILE_ALT"
        echo -e "${GREEN}  localhost.run: $URL_ALT${NC}"
        break
      fi
      sleep 1
    done
  fi

  # Notify if either changed (poll URLs from files)
  NEW_URL=$(cat "$URL_FILE" 2>/dev/null || echo "")
  NEW_URL_ALT=$(cat "$URL_FILE_ALT" 2>/dev/null || echo "")
  if [ -n "$NEW_URL" ] && [ -n "$NEW_URL_ALT" ]; then
    if [ "$NEW_URL" != "$URL" ] || [ "$NEW_URL_ALT" != "$URL_ALT" ]; then
      URL="$NEW_URL"
      URL_ALT="$NEW_URL_ALT"
      notify_ntfy "$URL" "$URL_ALT"
    fi
  fi

  sleep 5
done
