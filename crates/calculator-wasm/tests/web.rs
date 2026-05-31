//! Test suite for the Web and headless browsers.
//!
//! Run with:
//!   wasm-pack test --headless --firefox crates/calculator-wasm
//! drop `--headless` to run in a visible browser window).
//!

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use calculator_wasm::evaluate_expression;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn evaluates_addition() {
    assert_eq!(evaluate_expression("2 + 3").unwrap(), 5.0);
}

#[wasm_bindgen_test]
fn respects_precedence() {
    assert_eq!(evaluate_expression("1 + 2 * 3").unwrap(), 7.0);
}

#[wasm_bindgen_test]
fn respects_parentheses() {
    assert_eq!(evaluate_expression("(1 + 2) * 3").unwrap(), 9.0);
}

#[wasm_bindgen_test]
fn rejects_division_by_zero() {
    assert!(evaluate_expression("1 / 0").is_err());
}

#[wasm_bindgen_test]
fn rejects_syntax_error() {
    assert!(evaluate_expression("2 +").is_err());
}
