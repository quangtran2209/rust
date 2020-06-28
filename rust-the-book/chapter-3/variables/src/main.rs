use std::any::type_name;

fn main() {
    declare_a_mutable_variable();
    declare_a_constant();
    shadow_variables();
}

fn declare_a_mutable_variable() {
    // `mut` keyword stands for mutable
    // Declaring `mut` variable conveys the intention that other part of the code will be changing its value
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn declare_a_constant() {
    // Declaring a constant is required to declare its data type
    // A constant is always immutable
    // `Underscore` can be placed any where except at the beginning of the number
    const MAX_POINTS: u32 = 100_000;
    println!("The value of constant MAX_POINTS is: {}", MAX_POINTS);
}

fn shadow_variables() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";

    // type of `spaces` is &str
    println!("The type of spaces is: {}", type_of(spaces));

    // The different between `mut` and shadowing is that we can transform the variable
    // to become another type without having to declare another variable, and can reuse that name
    // After the transformation is completed, the variable is immutable
    // shadow `spaces` to become usize type
    let spaces = spaces.len();

    // type of `spaces` is now usize
    println!("The type of spaces is: {}", type_of(spaces));
}

// This shouldn't be used in production code as Rust is a strong static typing language
// You don't want to check types. You want to provide the types which will be checked by the compiler
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
