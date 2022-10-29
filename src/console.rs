use std::io::{stdin, stdout, Write};

pub fn print_line(msg: String) {
    print!("{} > ", msg);
    stdout().flush().unwrap();
}

pub fn read_line() -> String {
    let mut s: String = String::new();
    stdin().read_line(&mut s).expect("Failed to read line");
    {
        while s.ends_with('\n') || s.ends_with('\r') {
            s.pop();
        }
    }
    // TODO: trim whitespace from input
    return s;
}

#[allow(dead_code)]
pub fn verify_input(_matches_to: String) -> bool {
    // TODO: impl varify_input fn
    return true;
}
