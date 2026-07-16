# CID Bridge — universal MCP bridge for AI chatbots

> **L.ai · Bridge** — part of the [L.ai](https://github.com/nutypebuddha/lai) umbrella. *Verify, don't trust.*

<p align="center">
  <img src="https://codeberg.org/NutypeBuddha/cid/raw/branch/main/docs/logo.svg" width="120" alt="CID logo" />
</p>

[![License](https://img.shields.io/badge/license-unlicense-blue.svg)](LICENSE)
[![Wintermore Housekeeping](https://img.shields.io/badge/by-Wintermore%20Housekeeping-6366f1)](https://codeberg.org/NutypeBuddha)

Any AI chatbot (Grok, OpenAI, Anthropic, Mistral, Claude, etc.) can hook into [CID](https://codeberg.org/NutypeBuddha/cid) validation by pointing at this bridge.

## Architecture

```
AI Chatbot → /mcp → Platform Adapter → CID binary → Validated Output
                          ↓
              Grok · OpenAI · Anthropic · Mistral · Claude
```

The bridge translates each platform's message format into CID validation calls and returns structured results.

## Quick start

```bash
git clone https://codeberg.org/NutypeBuddha/cid-bridge.git
cd cid-bridge
npm install
CID_BINARY=../target/release/lai npm start
```

## API

### `POST /mcp`

Platform-aware validation. The bridge detects the caller's platform and routes accordingly.

```json
{
  "platform": "grok",
  "action": "validate",
  "data": { "text": "2 + 2 = 4", "context": "math" },
  "metadata": {}
}
```

**Supported platforms**: `grok`, `openai`, `anthropic`, `mistral`, `claude`, `generic`

**Actions**: `validate` (CID validation), `search` (knowledge base lookup)

### `POST /validate`

Direct CID validation — no platform routing.

```json
{ "text": "2 + 2 = 4", "context": "math" }
```

**Response**:
```json
{
  "success": true,
  "data": {
    "validated_text": "2+2=4",
    "confidence": 0.82,
    "passed": true,
    "fix_count": 0
  }
}
```

### `GET /health`

```json
{ "status": "ok", "service": "cid-bridge", "version": "1.0.0" }
```

## Deployment

### Production (systemd + nginx)

```bash
./deploy.sh
```

Sets up: systemd service, nginx reverse proxy on port 80, PM2 process manager.

### Docker

```bash
docker build -t cid-bridge .
docker run -p 3000:3000 -e CID_BINARY=/usr/local/bin/cid cid-bridge
```

### Kubernetes

```bash
kubectl apply -f k8s/deployment.yaml
```

### PM2 (any environment)

```bash
npm install -g pm2
CID_BINARY=/path/to/cid pm2 start src/index.js --name cid-bridge
pm2 save
pm2 startup
```

## Verification

```bash
curl -s http://localhost/health
curl -s -X POST http://localhost/validate \
  -H 'Content-Type: application/json' \
  -d '{"text":"2+2=4","context":"math"}'
curl -s -X POST http://localhost/mcp \
  -H 'Content-Type: application/json' \
  -d '{"platform":"grok","action":"validate","data":{"text":"E=mc^2","context":"fact"}}'
```

## CID Integration

The bridge shells out to the L.ai binary:

```bash
lai gate validate "<text>" <context>
```

Set `CID_BINARY` env var to point at your `lai` binary. Built via:

```bash
cargo build --release -p laverna    # produces target/release/lai
```

## License

Unlicense — public domain.

---

**Wintermore Housekeeping** — keeping LLMs in line.
