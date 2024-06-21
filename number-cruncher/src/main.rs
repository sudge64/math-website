mod math_basic;

fn main() {
    let a = 1.0;
    let b = 0.0;
    println!("Addition: {}", math_basic::addition(a, b));
    println!("Subtraction: {}", math_basic::subtraction(a, b));
    println!("Multiplication: {}", math_basic::multiplication(a, b));
    println!("Divison: {}", math_basic::division(a, b));
    println!("Modulo: {}", math_basic::modulo(a, b));
}
