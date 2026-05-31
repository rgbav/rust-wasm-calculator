# wasm-basic-calculator

A basic calculator that evaluates arithmetic expressions in Rust,
compiled to WebAssembly, and called from a React + TypeScript frontend.

Supported syntax: integers and decimals, `+ - * /`, and parentheses, with
standard operator precedence.

## Components

The workspace is split into two crates and a frontend:

- **`calculator-core`** — lexer, recursive-descent parser, and evaluator. No `wasm-bindgen` or JS types.
- **`calculator-wasm`** — a `wasm-bindgen` adapter that exposes
  `evaluate_expression` to JavaScript and maps `EvaluationError` onto
  `JsError`.
- **`frontend/`** — a Vite + React + TypeScript app that imports the
  generated wasm package and calls `evaluate_expression` on `=`.

## Prerequisites

- Rust stable (1.75+) with the `wasm32-unknown-unknown` target:
  `rustup target add wasm32-unknown-unknown`
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- Node.js 18+ and npm

## Build & run

From the repository root:

```bash
# 1. Compile the Rust crate to WebAssembly and emit a JS/TS package in
#    crates/calculator-wasm/pkg/.
wasm-pack build --target web crates/calculator-wasm

# 2. Install frontend dependencies. `calculator-wasm` is declared as a
#    file: dependency pointing at the pkg/ directory above.
cd frontend
npm install

# 3. Start the dev server.
npm run dev
```

## Tests

```bash
# Native unit tests for the parser and evaluator.
cargo test -p calculator-core

# Integration tests that exercise the wasm-bindgen boundary in a browser.
# Swap --firefox for --chrome/--safari as needed.
wasm-pack test --headless --firefox crates/calculator-wasm
```