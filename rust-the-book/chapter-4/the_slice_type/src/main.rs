fn main() {
    let s = "hello world";

    let first_word = first_word(s);

    println!("First word = `{}`", first_word);

    let second_word = second_word(s);

    println!("Second word = `{}`", second_word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &str) -> &str {

    let str_array: Vec<&str> = s.split(' ').collect();

    str_array.get(1).unwrap_or(&&s)
}
