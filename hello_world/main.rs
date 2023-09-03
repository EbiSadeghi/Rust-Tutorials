fn main() {
    let mut my_name = "Ebi";
    println!("{}", my_name);

    my_name = "Tom";
    println!("{}", my_name);

    const ONE_K: u32 = 11_000;
    const PI: f32 = 3.14159;

    let age: &str = "47";
    let character: char = 'a';

    println!("Signed: {}", u8::MAX);
    println!("Unsigned: {}", i8::MAX);

    println!("{}", character);

    println!("Hello, world!");
}