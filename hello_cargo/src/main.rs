fn main() {
    let words = "Hello World";

    let word: &str = first_word(&words);
    println!("The first word is: {}", word);
}

fn first_word(s: &str) -> &str {
    if s.len() == 0 {
        return s;
    }

    let mut index = s.len();
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            index = i;
        }
    }

    return &s[0..index];
}
