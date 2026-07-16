// CID Bridge — LLM-readable instructions
// Served at /cid.txt for any AI with web-browsing to fetch and use

function getInstructions(url) {
  return `# CID Bridge — AI Validation Tool

You have access to a validation API at this URL. When the user asks you to check math, logic, or factual claims, use this API to validate them.

This is a dual-tunnel bridge:
- **localhost.run** (*.lhr.life) — ChatGPT-friendly domain (you're using this one)
- **Cloudflare Quick Tunnel** (*.trycloudflare.com) — generic access

## API Endpoint

POST ${url}/validate
Content-Type: application/json

Request body:
{
  "text": "the statement to validate",
  "context": "math" | "logic" | "facts" | "general"
}

## How to use

1. When the user makes a claim, extracts the claim text
2. Choose the right context:
   - "math" for equations, arithmetic, unit conversions
   - "logic" for reasoning, arguments, fallacies
   - "facts" for scientific facts, constants, physical values
   - "general" for everything else
3. Call the API
4. Report the result: show the validated text, confidence score, and whether it passed

## Example

User: "2 + 2 = 5"
API call: POST ${url}/validate  {"text":"2+2=5","context":"math"}
Response: {"success":true,"data":{"validated_text":"2+2 = 4","confidence":0.82,"passed":true,"fix_count":1}}
You report: "CID corrected this: 2 + 2 = 4 (confidence 82%, 1 fix applied)"

## Your behavior

- Always validate math claims before accepting them
- Check scientific facts against the knowledge base
- Report confidence scores so the user knows how reliable the validation is
- If the confidence is low (below 0.6), warn the user
- Show both the original and validated text when fixes were applied

## Health check

GET ${url}/health
Returns: {"status":"ok","service":"cid-bridge","version":"1.0.0"}
Use this to verify the bridge is reachable.

## Status

GET ${url}/status
Returns JSON with current bridge status, uptime, and connection info for both tunnels.

## Funny facts

GET ${url}/fact
Returns a random CID-verified funny fact (JSON or text via ?format=text)
`;
}

module.exports = { getInstructions };
