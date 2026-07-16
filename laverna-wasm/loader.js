// Ł.AI · Proof — browser loader for laverna_wasm
// Initializes the WASM module and exposes a tiny demo harness.

import init, { evaluate, sha256, solve, verify } from "./laverna_wasm.js";

async function main() {
  await init();

  const out = document.getElementById("out");
  const run = (label, fn) => {
    try {
      const r = fn();
      out.textContent += `\n[${label}] ${JSON.stringify(r)}`;
    } catch (e) {
      out.textContent += `\n[${label}] error: ${e}`;
    }
  };

  run("evaluate 2 + 3 * 4", () => evaluate("2 + 3 * 4"));
  run("sha256 hello", () => sha256("hello"));
  run("solve knapsack", () =>
    solve(
      JSON.stringify({
        shape: "knapsack",
        items: [
          { id: "a", weight: 1, value: 2 },
          { id: "b", weight: 2, value: 5 },
          { id: "c", weight: 3, value: 6 },
        ],
        capacity: 4,
      })
    )
  );
}

main();
