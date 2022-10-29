#[path = "console.rs"]
mod console;

pub fn options(mut s: String) -> String {
    let data = ask_user();
    {
        // * Lowercase
        if data[0].to_lowercase().contains('n') {
            s = s.to_lowercase();
        }
    }
    {
        // * Ignore special chracters
        if data[1].to_lowercase().contains('y') {
            s = ignore_special_char(s);
        }
    }
    {
        // * Ignore numbers
        if data[2].to_lowercase().contains('y') {
            s = ignore_numbers(s);
        }
    }
    return s;
}

fn ask_user() -> Vec<String> {
    let msgs: [String; 3] = [
        "Case Sensative?(Y\\n)".to_string(),
        "Ignore special chracters?(Y\\n)".to_string(),
        "Ignore numbers?(Y\\n)".to_string(),
    ];
    {
        let mut answer = Vec::new();
        for each in msgs {
            console::print_line(each);
            answer.push(console::read_line());
        }
        return answer;
    }
}

fn ignore_special_char(mut s: String) -> String {
    let mut tmp_s: String = String::new();
    let mut i: usize = 0;
    for ch in s.chars() {
        if ch.is_alphanumeric() {
            tmp_s.insert(i, ch);
            i += 1;
        }
    }
    s = tmp_s;
    return s;
}

fn ignore_numbers(mut s: String) -> String {
    let mut tmp_s: String = String::new();
    let mut i: usize = 0;
    for ch in s.chars() {
        if !ch.is_numeric() {
            tmp_s.insert(i, ch);
            i += 1;
        }
    }
    s = tmp_s;
    return s;
}

// ! deprecated
#[allow(dead_code)]
fn ignore(mut s: String, logic: bool) -> String {
    let mut tmp_s: String = String::new();
    let mut i: usize = 0;
    for ch in s.chars() {
        if logic {
            tmp_s.insert(i, ch);
            i += 1;
        }
    }
    s = tmp_s;
    return s;
    /*
     * Impl this function in way that it combines
     * both ignore_special_char and ignore_numbers
     */
}
