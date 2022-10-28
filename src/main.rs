use std::io::{stdin, stdout, Write};

fn main() {
    let mut input: String = read_from_prompt("Enter a String>".to_string());
    input = options(input);
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

fn options(mut s: String) -> String {
    #[allow(unused_assignments)]
    let mut msg: String = String::new();
    {
        msg = "Is case sensitive?(Y\\n)>".to_string();
        let is_case_sensative: String = read_from_prompt(msg);
        if is_case_sensative.to_lowercase().contains('n') {
            s = s.to_lowercase();
        }
    }
    return s;
}
