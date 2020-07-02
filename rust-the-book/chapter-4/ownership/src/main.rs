fn main() {
    let s1 = String::from("Hello, world!");
    let s2 = s1;

    // This won't compile as s2 takes the ownership of s1 at line 3, so s1 is invalid
    // it also means that s1 was moved into s2 (s1 reference is invalidated in heap memory)
    // once s2 goes out of scope, it alone will free the memory and won't cause any memory safety bugs
    println!("s1 = {}", s1);

    // to create a deeply copy, use clone()
    // it copies the heap data, and stack data
    let s3 = s2.clone();

    println!("s2 = {}, s3 = {}", s2, s3);

    // x is an integer - known size at compile time, and stored directly on stack - so x wasn't moved into y after y assignment
    // integers are also annotated with Copy trait
    // any types are Copy won't be `moved` such as integers, boolean type, floating point types, characters, tuples with only Copy types
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
}

fn takes_ownership(a_string: String) { // a_string moves into scope
    println!("Value: {}", a_string) // after completing the function, a_string is cleaned up by `drop` function
}

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and move out to calling function
}

fn takes_and_gives_back_ownership(a_string: String) -> String { // a_string comes into scope
    println!("Value: {}", a_string);
    a_string // a_string is returned and move out to calling function
}
