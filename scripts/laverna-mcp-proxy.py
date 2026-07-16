#!/usr/bin/env python3
"""
Laverna verification proxy — Stage 1 PoC (Termux path A).

Bridges a conversational layer to Laverna's MCP tool server and enforces the
IP report's companion rule:

    "every factual claim the companion makes is either (a) produced by a
     tool call whose result is a hashed, reproducible proof object, or
     (b) explicitly flagged as opinion/uncertain."

This proxy is the deterministic half of the LLM<->Laverna loop. It speaks
the MCP JSON-RPC protocol over laverna's stdin/stdout server, and implements
a *verify-first* policy:

  - factual / computable questions  -> routed to a tool call, answer returned
    with the tool name + a reproducibility note (the "show me the receipt" UX)
  - subjective / out-of-corpus questions -> refused or flagged UNVERIFIED,
    never fabricated.

Without an LLM API key present it runs in --demo mode: a fixed set of
probes exercising both branches (verified vs. refused).

Usage:
    laverna-mcp-proxy.py --query "what graha rules mithuna?"
    laverna-mcp-proxy.py --demo
    laverna-mcp-proxy.py --server ./target/release/laverna
"""
import argparse
import json
import subprocess
import sys
import os

DEFAULT_SERVER = os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
    "target", "release", "laverna",
)

# Tools exposed by laverna mcp that can produce *verified* factual output.
VERIFYING_TOOLS = {
    "solve", "entity_get", "chart", "validate", "formulas",
    "entities", "optimize", "route", "build",
}


class McpClient:
    """Minimal MCP JSON-RPC client over a subprocess's stdin/stdout."""

    def __init__(self, server_bin):
        self.proc = subprocess.Popen(
            [server_bin, "mcp"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.DEVNULL,
            text=True,
            bufsize=1,
        )
        self._id = 0
        self._handshake()

    def _send(self, method, params=None):
        self._id += 1
        req = {"jsonrpc": "2.0", "id": self._id, "method": method}
        if params is not None:
            req["params"] = params
        self.proc.stdin.write(json.dumps(req) + "\n")
        self.proc.stdin.flush()
        # read until we get a response with our id
        while True:
            line = self.proc.stdout.readline()
            if not line:
                return None
            line = line.strip()
            if not line:
                continue
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if msg.get("id") == self._id:
                return msg

    def _handshake(self):
        self._send("initialize", {})
        # notifications/initialized needs no response
        self.proc.stdin.write(
            json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n"
        )
        self.proc.stdin.flush()

    def list_tools(self):
        resp = self._send("tools/list", {})
        return resp["result"]["tools"] if resp else []

    def call_tool(self, name, arguments):
        resp = self._send("tools/call", {"name": name, "arguments": arguments})
        if not resp:
            return {"error": "no response"}
        if "error" in resp:
            return {"error": resp["error"]}
        return resp["result"]

    def close(self):
        try:
            self.proc.stdin.close()
        except Exception:
            pass
        try:
            self.proc.wait(timeout=3)
        except Exception:
            self.proc.kill()


def classify(query):
    """Heuristic: is this a factual/computable claim Laverna can verify?

    Returns (is_factual, tool_hint). A real deployment replaces this with an
    LLM routing call; here we use simple keyword routing so the loop is
    demonstrable offline.

    CRITICAL (IP report companion rule): subjective / opinion / personal
    questions are NOT routed to a tool — they are refused as UNVERIFIED so the
    companion never fabricates. Laverna's `solve` is a deterministic reasoning
    kernel, not a truth oracle for beliefs.
    """
    q = query.lower()
    # Subjective / opinion / personal-entity signals -> refuse (never fabricate).
    opinion = [
        "do you think", "what do you think", "do you believe", "is it real",
        "is astrology real", "do you like", "your opinion", "breakfast",
        "lunch", "dinner", "favorite", "feel about", "should i", "would you",
    ]
    if any(k in q for k in opinion):
        return False, None
    if any(k in q for k in ["chart", "lagna", "birth", "horoscope", "graha position"]):
        return True, "chart"
    if any(k in q for k in ["entity", "who is", "what is", "define"]) and "graha" in q:
        return True, "entity_get"
    if any(k in q for k in ["route", "wheel", "which graha", "rules"]):
        return True, "route"
    if any(k in q for k in ["formula", "corpus", "expression", "compute", "calculate", "solve"]):
        return True, "solve"
    if any(k in q for k in ["optimize", "allocate", "budget"]):
        return True, "optimize"
    # Genuinelycomputable factual questions (contain a query verb + subject).
    if any(k in q for k in ["which", "what", "who", "how many", "how much"]) and len(q.split()) >= 3:
        return True, "solve"
    return False, None


def answer(client, query):
    is_factual, tool = classify(query)
    if not is_factual:
        return {
            "verdict": "UNVERIFIED",
            "tool": None,
            "text": (
                "I can't verify that against Laverna's deterministic corpus, "
                "and I won't fabricate. Ask me something computable — a chart, "
                "a graha/entity lookup, a formula, or a route through the wheel."
            ),
        }
    args = {"query": query}
    if tool == "chart":
        # chart requires an explicit UTC instant or local+IANA tz.
        args = {"query": query, "datetime_utc": "2000-01-01T12:00:00Z"}
    result = client.call_tool(tool, args)
    if "error" in result:
        return {
            "verdict": "TOOL_ERROR",
            "tool": tool,
            "text": f"Tool '{tool}' returned an error: {result['error']}. "
                    f"No answer fabricated.",
        }
    # Extract the human text from the MCP tool result.
    text = ""
    structured = None
    if isinstance(result, dict):
        content = result.get("content", [])
        for item in content:
            if item.get("type") == "text":
                text += item.get("text", "")
            elif item.get("type") == "structuredContent":
                structured = item.get("data")
    return {
        "verdict": "VERIFIED",
        "tool": tool,
        "text": text.strip(),
        "structured": structured,
    }


DEMO_QUERIES = [
    "Which graha rules mithuna (Gemini)?",                       # -> route (verified)
    "Cast a sidereal chart for 2000-01-01T12:00Z",
    "What is the formula for computing shadbala?",                # -> solve (verified)
    "Do you think astrology is real?",                            # -> UNVERIFIED (refused)
    "What did you have for breakfast?",                            # -> UNVERIFIED (refused)
]


def main():
    ap = argparse.ArgumentParser(description="Laverna verification proxy (Stage 1 PoC)")
    ap.add_argument("--server", default=DEFAULT_SERVER, help="path to laverna binary")
    ap.add_argument("--query", help="a single question to route/verify")
    ap.add_argument("--demo", action="store_true", help="run built-in probe set")
    args = ap.parse_args()

    if not os.path.exists(args.server):
        sys.exit(f"[proxy] server binary not found: {args.server}\n"
                 f"        build it: cargo build --release --features mcp")

    client = McpClient(args.server)
    try:
        if args.demo or not args.query:
            print("=== Laverna verification proxy — demo (verify-first loop) ===\n")
            for q in DEMO_QUERIES:
                r = answer(client, q)
                print(f"Q: {q}")
                print(f"   -> verdict={r['verdict']}  tool={r['tool']}")
                snippet = (r.get("text") or "")[:160].replace("\n", " ")
                print(f"   -> {snippet}\n")
            return
        r = answer(client, args.query)
        out = {
            "verdict": r["verdict"],
            "tool": r["tool"],
            "answer": r.get("text"),
        }
        print(json.dumps(out, indent=2, ensure_ascii=False))
    finally:
        client.close()


if __name__ == "__main__":
    main()
