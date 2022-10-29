pub mod console;

fn main() {
    let msg: String = "Enter a String:".to_string();
    console::print_line(msg);
    let input: String = console::read_line();
    if input == input.chars().rev().collect::<String>() {
        print!("Is Palindrome");
    } else {
        print!("Not a Palindrome");
    }
}