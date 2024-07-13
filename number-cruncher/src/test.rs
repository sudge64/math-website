use crate::shunting_yard::shunting_yard;

#[test]
fn success() {
    let answer: Vec<char> = vec!['3', '4', '2', '*', '1', '5', '-', '2', '3', '^', '^', '/', '+'];
    assert_eq!(answer, shunting_yard("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3".to_string()));
}
