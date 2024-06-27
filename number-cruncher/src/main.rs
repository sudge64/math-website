use std::{env, str::FromStr};

mod math_basic;
mod math_logic;

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = f32::from_str(&args[1]).unwrap();
    let b = f32::from_str(&args[2]).unwrap();
    let opt = String::from_str(&args[3]).unwrap();

    let bool_a = logic_match(a as u8);
    let bool_b = logic_match(b as u8);

    match opt.as_str() {
        "+" => {
            println!("Addition: {}", math_basic::addition(a, b));
        }
        "-" => {
            println!("Subtraction: {}", math_basic::subtraction(a, b));
        }
        "*" => {
            println!("Multiplication: {}", math_basic::multiplication(a, b));
        }
        "/" => {
            println!("Divison: {:?}", math_basic::division(a, b));
        }
        "%" => {
            println!("Modulo: {:?}", math_basic::modulo(a, b));
        }
        "AND" => {
            println!("AND: {:?}", math_logic::and_logic(bool_a, bool_b));
        }
        "OR" => {
            println!("OR: {:?}", math_logic::or_logic(bool_a, bool_b));
        }
        "NAND" => {
            println!("NAND: {:?}", math_logic::nand_logic(bool_a, bool_b));
        }
        "NOR" => {
            println!("NOR: {:?}", math_logic::nor_logic(bool_a, bool_b));
        }
        "XOR" => {
            println!("XOR: {:?}", math_logic::xor_logic(bool_a, bool_b));
        }
        "NOT" => {
            println!("NOT: {:?}", math_logic::not_logic(bool_a));
            println!("NOT: {:?}", math_logic::not_logic(bool_b));
        }
        _ => {
            panic!("ERROR: Unrecognized operator.");
        }
    }
}

fn logic_match(operand: u8) -> bool {
    match operand {
        0 => false,
        1 => true,
        _ => {
            panic!("ERROR: Operand not 0 or 1.");
        }
    }
}
