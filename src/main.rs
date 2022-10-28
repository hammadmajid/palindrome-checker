use std::io::{stdin, stdout, Write};
fn main() {
    let input: String = read_from_prompt();
    print!("Your Input: {input}");
}

fn read_from_prompt() -> String {
    print!("Enter a string: ");
    stdout().flush().unwrap();
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
    return s;
}
