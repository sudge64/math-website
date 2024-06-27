use std::{env, str::FromStr};

mod math_basic;
mod math_logic;
mod math_binary;

fn main() {
    let args: Vec<String> = env::args().collect();

    let a = f32::from_str(&args[1]).unwrap();
    let b = f32::from_str(&args[2]).unwrap();
    let opt = String::from_str(&args[3]).unwrap();

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
            println!("AND: {:?}", math_logic::and_logic(logic_match(a as u8), logic_match(b as u8)));
        }
        "OR" => {
            println!("OR: {:?}", math_logic::or_logic(logic_match(a as u8), logic_match(b as u8)));
        }
        "NAND" => {
            println!("NAND: {:?}", math_logic::nand_logic(logic_match(a as u8), logic_match(b as u8)));
        }
        "NOR" => {
            println!("NOR: {:?}", math_logic::nor_logic(logic_match(a as u8), logic_match(b as u8)));
        }
        "XOR" => {
            println!("XOR: {:?}", math_logic::xor_logic(logic_match(a as u8), logic_match(b as u8)));
        }
        "NOT" => {
            println!("NOT: {:?}", math_logic::not_logic(logic_match(a as u8)));
            println!("NOT: {:?}", math_logic::not_logic(logic_match(b as u8)));
        }
        "BIN" => {
            println!("{:?}", math_binary::decimal_to_binary(a as i32));
            println!("{:?}", math_binary::decimal_to_binary(b as i32));
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
