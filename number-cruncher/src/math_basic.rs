use core::panic;
use std::error::Error;

pub fn addition(a: f32, b: f32) -> f32 {
    a + b
}
pub fn subtraction(a: f32, b: f32) -> f32 {
    a - b
}
pub fn multiplication(a: f32, b: f32) -> f32 {
    a * b
}
pub fn division(a: f32, b: f32) -> Result<f32, &'static str> {
    if b != 0.0 {
        Ok(a/b)
    } else {
        Err("Division by zero")
    }
}
pub fn modulo(a: f32, b: f32) -> Result<f32, &'static str> {
    if b != 0.0 {
        Ok(a%b)
    } else {
        Err("Division by zero")
    }
}
