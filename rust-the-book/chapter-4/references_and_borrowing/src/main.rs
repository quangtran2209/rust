fn main() {
    let s1 = String::from("hello");

    // `&s1` is a reference refers to value of s1, but does not own it
    // so after this reference goes out of `calculate_length` function scope, nothing happens - `s1` won't be dropped
    let len = calculate_length(&s1);

    println!("The length of string `{}` is {}", s1, len)
}

fn calculate_length(str: &String) -> usize {
    str.len()
}
