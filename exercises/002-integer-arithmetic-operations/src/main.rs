fn main() {
    let number_1 = 15;
    // number_1 = 2 error: number_1 is immutable
    let mut number_2 = 5; // number_2 is now mutable
    number_2 = 2; 

    let addition = number_1 + number_2;
    let subtraction = number_1 - number_2;
    let multiplication = number_1 * number_2;
    let division = number_1 / number_2; // ignores fractional part
    let remainder = number_1 % number_2;

    println!("Addition Result: {addition}");
    println!("Subtraction Result: {subtraction}");
    println!("Multiplication Result: {multiplication}");
    println!("Division Result: {division}");
    println!("Remainder Result: {remainder}");
}
