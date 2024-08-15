use crate::shunting_yard::shunting_yard;
use crate::evaluate_rpn;

#[test]
fn shunting_yard_success() {
    let answer: Vec<String> = vec!["3".to_string(), "4".to_string(), "2".to_string(), "*".to_string(), "1".to_string(), "5".to_string(), "-".to_string(), "2".to_string(), "3".to_string(), "^".to_string(), "^".to_string(), "/".to_string(), "+".to_string()];
    assert_eq!(answer, shunting_yard("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3".to_string()));
}

#[test]
fn rpn_success() {
    let answer: i64 = 7;
    let output_queue: Vec<String> = vec!["1".to_string(), "2".to_string(), "3".to_string(), "*".to_string(), "+".to_string()];
    assert_eq!(answer, evaluate_rpn(output_queue));
}
