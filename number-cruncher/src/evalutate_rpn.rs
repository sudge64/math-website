/*
* Original Author: C.J. Wade
* License: T.B.D.
* Description: Evalutates RPN stacks.
*               I hacked away at this on my own, but recieved help from
*               this article: https://medium.com/nplan/aoc-2020-day-18-shunting-yard-algorithm-with-rust-6439cb02dbad
*               I would like to point out that the only difference between
*               his solution and mine was me only having one set of `a` & `b`
*               before the `match` statement. This was causing the complier
*               to panic when it read a `None` from the empty stack.
*/

pub fn evaluate_rpn(output_queue: Vec<String>) -> f64 {
    let mut stack: Vec<f64> = Vec::new();

    for token in output_queue.into_iter() {
        // My a & b were here.
        match token.as_str() {
            "+" => {
                let b: f64 = stack.pop().unwrap();
                let a: f64 = stack.pop().unwrap();
                stack.push(a + b);
            }
            "-" => {
                let b: f64 = stack.pop().unwrap();
                let a: f64 = stack.pop().unwrap();
                stack.push(a - b);
            }
            "*" => {
                let b: f64 = stack.pop().unwrap();
                let a: f64 = stack.pop().unwrap();
                stack.push(a * b);
            }
            "/" => {
                let b: f64 = stack.pop().unwrap();
                let a: f64 = stack.pop().unwrap();
                stack.push(a / b);
            }
            "%" => {
                let b: f64 = stack.pop().unwrap();
                let a: f64 = stack.pop().unwrap();
                stack.push(a % b);
            }
            "^" => {
                let b: f64 = stack.pop().unwrap();
                let a: f64 = stack.pop().unwrap();
                stack.push(f64::powf(a, b));
            }
            _ => {
                stack.push(token.parse().unwrap());
            }
        }
    }

    assert!(stack.len() == 1);
    stack[0]
}
