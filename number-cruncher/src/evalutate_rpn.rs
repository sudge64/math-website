/* 
* Original Author: C.J. Wade
* License: T.B.D.
* Description: 
*/

pub fn evaluate_rpn(output_queue: Vec<char>) -> u32 {
    let mut stack: Vec<u32> = Vec::new();

    for token in output_queue.into_iter() {
        match token {
            '+' => {
                let a: u32 = stack.pop().unwrap();
                let b: u32 = stack.pop().unwrap();
                stack.push(a + b);
            }
            '-' => {
                let a: u32 = stack.pop().unwrap();
                let b: u32 = stack.pop().unwrap();
                stack.push(a - b);
            }
            '*' => {
                let a: u32 = stack.pop().unwrap();
                let b: u32 = stack.pop().unwrap();
                stack.push(a * b);
            }
            '/' => {
                let a: u32 = stack.pop().unwrap();
                let b: u32 = stack.pop().unwrap();
                stack.push(a / b);
            }
            '%' => {
                let a: u32 = stack.pop().unwrap();
                let b: u32 = stack.pop().unwrap();
                stack.push(a % b);
            }
            '^' => {
                let a: u32 = stack.pop().unwrap();
                let b: u32 = stack.pop().unwrap();
                stack.push(u32::pow(a, b));
            }
            _ => {
                stack.push(token.to_digit(10).unwrap() as u32);
            }
        }
    }

    assert!(stack.len() == 1);
    stack[0]
}
