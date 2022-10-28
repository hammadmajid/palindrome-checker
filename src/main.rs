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
    {
        msg = "Ignore special chracters?(Y\\n)?".to_string();
        let ignore_special_chars: String = read_from_prompt(msg);
        if ignore_special_chars.to_lowercase().contains('y') {
            let mut tmp_s: String = String::new();
            let mut i: usize = 0;
            for ch in s.chars() {
                // #[allow(unused_assignments)]
                if ch.is_alphanumeric() {
                    tmp_s.insert(i, ch);
                    i += 1;
                }
            }
            s = tmp_s;
        }
    }
    {
        msg = "Ignore numbers?(Y\\n)>".to_string();
        let igonore_numbers: String = read_from_prompt(msg);
        if igonore_numbers.to_lowercase().contains('y') {
            if igonore_numbers.to_lowercase().contains('y') {
                let mut tmp_s: String = String::new();
                let mut i: usize = 0;
                for ch in s.chars() {
                    if !ch.is_numeric() {
                        tmp_s.insert(i, ch);
                        i += 1;
                    }
                }
                s = tmp_s;
            }
        }
    }
    return s;
}
