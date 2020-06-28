use std::any::type_name;

fn main() {
    chapter_3_1_variables_and_mutability();
    chapter_3_2_data_types();
}

fn chapter_3_1_variables_and_mutability() {
    declare_a_mutable_variable();
    declare_a_constant();
    shadow_variables();
}

fn chapter_3_2_data_types() {
    declare_scalar_types();
    declare_compound_types();
    declare_array_type();
}

fn declare_scalar_types() {
    // a scalar type represent a single value
    // there are 4 primary scalar types in Rust:

    // integer: signed or unsigned
    // they refer to whether it's possible for a number to be positive or negative
    // i.e `i8` (twos complement notation) can store numbers from -(2^(n-1)) to (2^(n-1) - 1) = -(2^7) to (2^7 - 1)
    // while `u8` can only store numbers from 0 to (2^n - 1) = (2^8 - 1)
    // Which type to use? `i32` is default in Rust and generally the fastest even on 64-bit systems
    let signed_integer: i32 = 32;
    let unsigned_integer: u32 = 64;

    println!(
        "These are signed integer and unsigned integer respectively: {}, {}",
        signed_integer, unsigned_integer
    );

    // floating-point number: 2 primitive types f32 & f64
    // f32 is single-precision float
    // f64 is double-precision float
    let third_two_bits_floating_number = 64.0; // f64
    let sixty_four_bits_floating_number: f32 = 32.0;

    println!(
        "Floating point numbers: {}, {}",
        third_two_bits_floating_number, sixty_four_bits_floating_number
    );

    // boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("Boolean variables: {}, {}", t, f);

    // character
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("Characters: {}, {}, {}", c, z, heart_eyed_cat);
}

fn declare_compound_types() {
    // The tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("Tuple values: {}, {}, {}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;

    println!("Tuple part 1 - {}", x); // 500
    println!("Tuple part 2 - {}", y); // 6.4
    println!("Tuple part 3 - {}", z); // 1
}

fn declare_array_type() {
    // Arrays are useful when you want your data allocated on the `stack` rather than the `heap`
    // Declare an array can contain max 5 i32 integers
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Array `a`: {:?}", a);

    // Declare an array that contain 6 elements with the same specified value, i.e. 6
    let b = [4; 6];

    println!("Array `b`: {:?}", b);

    // Accessing array
    let first_elem = a[0];
    let second_elem = a[1];

    println!(
        "The first two elements of array `a`: {}, {}",
        first_elem, second_elem
    )
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
