fn main() {
    let s1 = String::from("hello");

    // `&s1` is a reference refers to value of s1, but does not own it
    // so after this reference goes out of `calculate_length` function scope, nothing happens - `s1` won't be dropped
    let len = calculate_length(&s1);

    println!("The length of string `{}` is {}", s1, len);

    let mut s2 = String::from("Hello");

    change_reference_value(&mut s2);

    println!("The value of `s2` is {}", s2);

    // s3 is borrowing the reference of s2
    let s3 = &mut s2;

    // s4 tries to borrow the reference of s2 one more time but it is not allowed here
    // the reference can only be borrowed once at a time
    // hence, it won't compile here
    let s4 = &mut s2;

    println!("`s3` = {}, `s4` = {}", s3, s4);

    let s5 = dangle();

    println!("`s5` = {}", s5);
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change_reference_value(reference_str: &mut String) {
    reference_str.push_str(", world!")
}

// create a function to return a dangling pointer - a pointer references to a location in memory
// that may have been given to someone else, by freeing that memory while preserving a pointer to that memory
fn dangle() -> &String {
    let s = String::from("Dangle");
    &s
} // s goes out of scope here when dangle() is finished, and will be deallocated