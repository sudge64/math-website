use crate::shunting_yard::shunting_yard;
use crate::evaluate_rpn;

#[cfg(test)]

#[test]
fn shunting_yard_success() {
    let answer: Vec<char> = vec!['3', '4', '2', '*', '1', '5', '-', '2', '3', '^', '^', '/', '+'];
    assert_eq!(answer, shunting_yard("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3".to_string()));
}

#[test]
fn rpn_success() {
    let answer: u32 = 7;
    let output_queue: Vec<char> = vec!['1', '2', '3', '*', '+'];
    assert_eq!(answer, evaluate_rpn(output_queue));
}
