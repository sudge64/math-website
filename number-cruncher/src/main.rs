use clap::Parser;
use evalutate_rpn::evaluate_rpn;

// mod math_basic;
// mod math_logic;
// mod math_binary;
mod shunting_yard;
mod evalutate_rpn;

#[cfg(test)]
mod test;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/*struct Args {
    a: f32,
    b: f32,
    opt: String
}*/
struct Args {
    math_string: String,
}

fn main() {
    let args = Args::parse();
    let math_string = args.math_string;
    let output_queue = shunting_yard::shunting_yard(math_string);
    println!("output_queue: {:?}", output_queue);
    let result = evaluate_rpn(output_queue);
    println!("{:?}", result);

    // let a = args.a;
    // let b = args.b;
    // let opt = args.opt;

    /*match opt.as_str() {
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
            math_binary::decimal_to_binary(a as i32);
            println!();
            math_binary::decimal_to_binary(b as i32);
            println!();
        }
        _ => {
            panic!("ERROR: Unrecognized operator.");
        }
    }*/
}

/* fn logic_match(operand: u8) -> bool {
    match operand {
        0 => false,
        1 => true,
        _ => {
            panic!("ERROR: Operand not 0 or 1.");
        }
    }
}*/
