use std::io::{stdin, stdout, Write};
fn main() {
    #[allow(unused_mut)]
    let mut input: String = read_from_prompt("Enter a String>".to_string());
    print!("Your Input: {}", input);
}

fn read_from_prompt(msg: String) -> String {
    {
        print!("{} ", msg);
        stdout().flush().unwrap();
    }
    {
        let mut s: String = String::new();
        stdin().read_line(&mut s).expect("Failed to read line");
        {
            while s.ends_with('\n') || s.ends_with('\r') {
                s.pop();
            }
        }
        return s;
    }
}
