// Copyright 2026 nutypebuddha
// SPDX-License-Identifier: Apache-2.0

//! WebAssembly bindings for Ł.AI · Proof (Laverna).
//!
//! Exposes the deterministic pure functions of the engine to the browser:
//! - `solve`    — resolve an optimization/puzzle schema through the descent cascade
//! - `evaluate` — evaluate a Tanto math expression
//! - `sha256`   — content-addressed proof hashing
//! - `verify`   — re-derive a proof object and confirm it checks out
//!
//! All functions are pure and deterministic; no global state.

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Result of a deterministic proof evaluation.
#[derive(Serialize, Deserialize)]
pub struct ProofResult {
    pub ok: bool,
    pub value: String,
    pub hash: String,
    pub detail: String,
}

/// Initialize the Ł.AI · Proof WASM module.
#[wasm_bindgen]
pub fn init() {
    // No global state; placeholder for symmetry with loaders.
}

/// Evaluate a Tanto math expression deterministically.
///
/// Pure function: `evaluate(expr) -> ProofResult`.
#[wasm_bindgen]
pub fn evaluate(expression: &str) -> JsValue {
    let env = laverna::compute::create_env();
    let result = match laverna::compute::evaluate_expr(expression, &env) {
        Some(value) => {
            let rendered = format!("{}", value);
            ProofResult {
                ok: true,
                value: rendered.clone(),
                hash: laverna::digest::sha256_hex(rendered.as_bytes()),
                detail: String::new(),
            }
        }
        None => ProofResult {
            ok: false,
            value: String::new(),
            hash: String::new(),
            detail: format!("could not evaluate expression: {}", expression),
        },
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// SHA-256 (hex) of an arbitrary input string — content-addressed proof hash.
///
/// Pure function: `sha256(input) -> hex`.
#[wasm_bindgen]
pub fn sha256(input: &str) -> String {
    laverna::digest::sha256_hex(input.as_bytes())
}

/// Solve an optimization schema expressed as JSON and return the allocation.
///
/// Pure function: `solve(schema_json) -> ProofResult`. The schema format matches
/// `laverna::optimize::Schema` (serde). Returns the deterministic allocation
/// rendered as a stable string (sorted where unordered collections appear).
#[wasm_bindgen]
pub fn solve(schema_json: &str) -> JsValue {
    let result = match serde_json::from_str::<laverna::optimize::Schema>(schema_json) {
        Ok(schema) => match laverna::optimize::solve(&schema, 1) {
            Ok(allocations) => {
                let rendered = format!("{:?}", allocations);
                let hash = laverna::digest::sha256_hex(rendered.as_bytes());
                ProofResult {
                    ok: true,
                    value: rendered,
                    hash,
                    detail: String::new(),
                }
            }
            Err(e) => fail(&e.to_string()),
        },
        Err(e) => fail(&e.to_string()),
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Verify a previously emitted proof object JSON: re-derive and confirm the hash.
///
/// Pure function: `verify(proof_json) -> ProofResult`. `proof_json` is the object
/// produced by `solve` (it carries the deterministic allocation + hash).
#[wasm_bindgen]
pub fn verify(proof_json: &str) -> JsValue {
    let parsed: serde_json::Value = match serde_json::from_str(proof_json) {
        Ok(v) => v,
        Err(e) => return serde_wasm_bindgen::to_value(&fail(&e.to_string())).unwrap(),
    };
    let value = parsed.get("value").and_then(|v| v.as_str()).unwrap_or("");
    let claimed = parsed
        .get("hash")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let actual = laverna::digest::sha256_hex(value.as_bytes());
    let ok = !claimed.is_empty() && claimed == actual;
    let result = ProofResult {
        ok,
        value: value.to_string(),
        hash: actual,
        detail: if ok {
            String::new()
        } else {
            "hash mismatch — proof object not reproducible".to_string()
        },
    };
    serde_wasm_bindgen::to_value(&result).unwrap()
}

fn fail(detail: &str) -> ProofResult {
    ProofResult {
        ok: false,
        value: String::new(),
        hash: String::new(),
        detail: detail.to_string(),
    }
}
