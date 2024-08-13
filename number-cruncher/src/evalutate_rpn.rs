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

pub fn evaluate_rpn(output_queue: Vec<char>) -> i64 {
    let mut stack: Vec<i64> = Vec::new();

    for token in output_queue.into_iter() {
        // My a & b were here.
        match token {
            '+' => {
                let b: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.push(a + b);
            }
            '-' => {
                let b: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.push(a - b);
            }
            '*' => {
                let b: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.push(a * b);
            }
            '/' => {
                let b: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.push(a / b);
            }
            '%' => {
                let b: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.push(a % b);
            }
            '^' => {
                let b: i64 = stack.pop().unwrap();
                let a: i64 = stack.pop().unwrap();
                stack.push(i64::pow(a, b as u32));
            }
            _ => {
                stack.push(token.to_digit(10).unwrap() as i64);
            }
        }
    }

    assert!(stack.len() == 1);
    stack[0]
}
