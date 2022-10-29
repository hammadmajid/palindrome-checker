pub mod console;
pub mod options;
fn main() {
    let msg: String = "Enter a String:".to_string();
    console::print_line(msg);
    let mut input: String = console::read_line();
    input = options::options(input);
    if input == input.chars().rev().collect::<String>() {
        print!("Is Palindrome");
    } else {
        print!("Not a Palindrome");
    }
}
