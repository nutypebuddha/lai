# CID Bridge — AI Chat Integration Guide

Connect any AI chatbot to CID validation in under 60 seconds.

The bridge provides **dual tunnels**:
- **🚀 localhost.run** (`*.lhr.life`) — ChatGPT-friendly domain, use for Custom GPT Actions
- **🌩️ Cloudflare Quick Tunnel** (`*.trycloudflare.com`) — generic access, may be blocked by ChatGPT

Your bridge URLs change with each tunnel restart. Run `./cid-share.sh` to get new URLs.

---

## Grok (xAI) — Native MCP

**Settings → MCP Servers → Add Server:**

| Field | Value |
|-------|-------|
| Name | `CID Bridge` |
| URL | `https://your-bridge.lhr.life/mcp` |

Once connected, Grok automatically validates math/logic/facts through CID in every conversation.

---

## ChatGPT — Custom GPT (Actions)

> ⚠️ **Important**: ChatGPT blocks `*.trycloudflare.com` domains. Use the localhost.run URL (`*.lhr.life`) for Custom GPT Actions.

1. **Create a Custom GPT** → **Configure**
2. **Add Action** → Import this OpenAPI schema:

```yaml
openapi: 3.1.0
info:
  title: CID Bridge
  description: Calibrated Inference Device — validate math, logic, facts
  version: 1.0.0
servers:
  - url: https://your-bridge.lhr.life
paths:
  /validate:
    post:
      summary: Validate any text through CID
      operationId: cidValidate
      requestBody:
        required: true
        content:
          application/json:
            schema:
              type: object
              required: [text]
              properties:
                text:
                  type: string
                  description: Text to validate (math, logic, facts)
                context:
                  type: string
                  enum: [math, logic, facts, general]
                  default: general
      responses:
        '200':
          description: Validation result
          content:
            application/json:
              schema:
                type: object
                properties:
                  success:
                    type: boolean
                  data:
                    type: object
                    properties:
                      validated_text:
                        type: string
                      confidence:
                        type: number
                      passed:
                        type: boolean
                      fix_count:
                        type: integer
  /health:
    get:
      summary: Health check
      operationId: healthCheck
      responses:
        '200':
          description: OK
```

3. **Add Instructions** to your GPT:
```
You have access to CID (Calibrated Inference Device) validation.
Use cidValidate() whenever the user asks you to check math, logic, or factual claims.
Show the validation result including confidence score.
```

---

## Claude Desktop — MCP Configuration

Edit `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "cid-bridge": {
      "type": "http",
      "url": "https://your-bridge.lhr.life/mcp"
    }
  }
}
```

---

## Claude Code (CLI)

```bash
claude mcp add cid-bridge --type=http --url=https://your-bridge.lhr.life/mcp
```

---

## Open WebUI

**Settings → Connections → MCP Servers → Add:**

| Field | Value |
|-------|-------|
| Name | `CID Bridge` |
| URL | `https://your-bridge.lhr.life/mcp` |

---

## Direct API (any platform)

If your AI supports custom API calls or function calling, use:

```bash
# Validate (use the localhost.run URL for best compatibility)
curl -X POST https://your-bridge.lhr.life/validate \
  -H 'Content-Type: application/json' \
  -d '{"text":"2+2=4","context":"math"}'

# Response:
# {"success":true,"data":{"validated_text":"2+2=4","confidence":0.82,"passed":true,"fix_count":0}}
```

Example function definition for function-calling models:

```json
{
  "name": "cid_validate",
  "description": "Validate text through CID (Calibrated Inference Device)",
  "parameters": {
    "type": "object",
    "properties": {
      "text": {
        "type": "string",
        "description": "Text to validate"
      },
      "context": {
        "type": "string",
        "enum": ["math", "logic", "facts", "general"],
        "description": "Validation domain"
      }
    },
    "required": ["text"]
  }
}
```

---

## Quick Reference

| AI Platform | Method | Setup Time |
|------------|--------|------------|
| Grok | Native MCP | 30s |
| ChatGPT | Custom GPT + Actions | 2 min |
| Claude Desktop | MCP config | 30s |
| Claude Code | CLI command | 10s |
| Open WebUI | MCP Settings | 30s |
| Any API-compatible | Function calling | 1 min |

---

*Built by Wintermore Housekeeping — keeping LLMs in line.*
