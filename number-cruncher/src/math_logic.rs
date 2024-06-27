pub fn and_logic(a: bool, b: bool) -> bool {
    a && b
}
pub fn or_logic(a: bool, b: bool) -> bool {
    a || b
}
pub fn nand_logic(a: bool, b: bool) -> bool {
    !(a && b)
}
pub fn nor_logic(a: bool, b: bool) -> bool {
    !(a || b)
}
pub fn xor_logic(a: bool, b: bool) -> bool {
    a ^ b
}
pub fn not_logic(a: bool) -> bool {
    !a
}
