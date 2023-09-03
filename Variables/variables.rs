#[allow(unused)]
// Mutability
// > let        - represents a memory location - value inwhich cannot be changed
// > let mut    - represents a memory location - value inwhich can be changed
// > const      - represents a value
// > static     - will be discussed later

// Signed integers:     i8, i16, i32, i64, i128, isize  = -2, -1, 0, 1, 2
// Unsigned integer:    u8, u16, u32, u64, u128, usize  = 0, 1, 2, 3, 4
// Float:               f32, f64                        = 1.245832
// Character:           char                            = '?'
// String:              str                             = "?...?"
// Boolean:             bool                            = true or false

// isize and usize are machine dependent.

fn main () {
    let mut my_name = "Ebi";

    println!("My name is: {}", my_name);

    my_name = "Tom";

    println!("My name is: {}", my_name);

    const PI: f32 = 3.14159;

    println!("Signed: {}, Unsigned {}", i8::MAX, u8::MAX);
    println!("3/2 = {}", 3/2);
}