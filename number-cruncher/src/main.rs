use std::{env, str::FromStr};

mod math_basic;

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = f32::from_str(&args[1]).unwrap();
    let b = f32::from_str(&args[2]).unwrap();
    let opt = char::from_str(&args[3]).unwrap();

    match opt {
        '+' => {
            println!("Addition: {}", math_basic::addition(a, b));
        },
        '-' => {
            println!("Subtraction: {}", math_basic::subtraction(a, b));
        },
        '*' => {
            println!("Multiplication: {}", math_basic::multiplication(a, b));
        },
        '/' => {
            println!("Divison: {:?}", math_basic::division(a, b));
        },
        '%' => {
            println!("Modulo: {:?}", math_basic::modulo(a, b));
        },
        _ => {
            panic!("ERROR: Unrecognized operator.");
        }
    }
}
