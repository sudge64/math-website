/* 
* Original Author: C.J. Wade
* License: T.B.D.
* Description: Implements Dijkstra's Shunting Yard algorithm as detailed from 
*              psuedo code written on [Wikipedia](https://en.wikipedia.org/wiki/Shunting_yard_algorithm)
*/
use core::panic;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Associativity {
    Left,
    Right,
}

#[derive(Debug)]
struct Operator {
    precedence: i32,
    associativity: Associativity,
}

pub fn shunting_yard(math_string: String) -> Vec<char> {
    let operators: HashMap<char, Operator> = {
        // Precendence values decided from this table: https://en.wikipedia.org/wiki/Shunting_yard_algorithm#Detailed_examples
        let mut m = HashMap::new();
        m.insert(
            '+',
            Operator {
                precedence: 2,
                associativity: Associativity::Left,
            },
        );
        m.insert(
            '-',
            Operator {
                precedence: 1,
                associativity: Associativity::Left,
            },
        );
        m.insert(
            '*',
            Operator {
                precedence: 4,
                associativity: Associativity::Left,
            },
        );
        m.insert(
            '/',
            Operator {
                precedence: 3,
                associativity: Associativity::Left,
            },
        );
        m.insert(
            '%',
            Operator {
                precedence: 3,
                associativity: Associativity::Left,
            },
        );
        m.insert(
            '^',
            Operator {
                precedence: 5,
                associativity: Associativity::Right,
            },
        );
        m.insert(
            '(',
            Operator {
                precedence: 0,
                associativity: Associativity::Left,
            },
        );
        m.insert(
            ')',
            Operator {
                precedence: 0,
                associativity: Associativity::Left,
            },
        );
        m
    };
    let mut output_queue: Vec<char> = Vec::new();
    let mut operator_stack: Vec<char> = Vec::new();

    // while there are tokens to be read:
    // read a token
    for token in math_string.chars() {
        // if the token is:
        // - a number:
        if token.is_numeric() {
            // put it into the output queue
            output_queue.push(token);
        } else if token == ' ' {
            continue;
        // - a function:
        // push it onto the operator stack
        // - an operator o1:
        } else if let Some(operator) = operators.get(&token) {
            if token == '(' {
                operator_stack.push(token);
            } else if token == ')' {
                while let Some(top) = operator_stack.pop() {
                    match top {
                        '(' => {
                            break;
                        }
                        _ => {
                            output_queue.push(top);
                        }
                    }
                }
            } else {
            // while (
                // there is an operator o2 at the top of the operator stack which is not a left parenthesis,
                // and (o2 has greater precedence than o1 or (o1 and o2 have the same precedence and o1 is left-associative))
            // ):
                while let Some(&o2) = operator_stack.last() {
                    let o2_operator = operators.get(&o2).unwrap();
                    if o2 != '('
                        && (o2_operator.precedence > operator.precedence
                            || (o2_operator.precedence == operator.precedence
                                && operator.associativity == Associativity::Left))
                    {
                        // pop o2 from the operator stack into the output queue
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                // push o1 onto the operator stack
                operator_stack.push(token);
            }
        //- a ",":
        } else if token == ',' {
            // while the operator at the top of the operator stack is not a left parenthesis:
            let popped_operator = operator_stack.pop().unwrap();
            if popped_operator != '(' {
                // pop the operator from the operator stack into the output queue
                output_queue.push(popped_operator);
            } else {
                break;
            }
        // - a left parenthesis (i.e. "("):
        } else if token == '(' {
            // push it onto the operator stack
            operator_stack.push(token);
        // - a right parenthesis (i.e. ")"):
        } else if token == ')' {
            // while the operator at the top of the operator stack is not a left parenthesis:
            let popped_operator = operator_stack.pop().unwrap();
            if popped_operator != '(' {
                // {assert the operator stack is not empty}
                if operator_stack.is_empty() == false {
                    // pop the operator from the operator stack into the output queue
                    output_queue.push(popped_operator);
                } else {
                    panic!("ERROR: Stack is empty somehow?");
                }
            } else {
                // pop the left parenthesis from the operator stack and discard it
                operator_stack.pop();
            }
            // if there is a function token at the top of the operator stack, then:
                // pop the function from the operator stack into the output queue
        } else {
            panic!("ERROR: Unrecognized operator.");
        }
    }

    // while there are tokens on the operator stack:
    operator_stack.clone().into_iter().for_each(|_| {
        let popped_operator = operator_stack.pop().unwrap();
        // {assert the operator on top of the stack is not a (left) parenthesis}
        if popped_operator != '(' {
            // pop the operator from the operator stack onto the output queue
            output_queue.push(popped_operator);
        }
    });

    // println!("OUTPUT QUEUE: {:?}", output_queue);
    // println!("OPERATOR STACK: {:?}", operator_stack);
    output_queue
}
