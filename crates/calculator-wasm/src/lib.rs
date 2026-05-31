use calculator_core::evaluate_str_expression;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate_expression(expr: &str) -> Result<f64, JsError> {
    evaluate_str_expression(expr).map_err(|e| JsError::new(&e.to_string()))
}
