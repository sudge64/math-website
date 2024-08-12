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

pub fn evaluate_rpn(output_queue: Vec<char>) -> u32 {
    let mut stack: Vec<u32> = Vec::new();

    for token in output_queue.into_iter() {
        // My a & b were here.
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
