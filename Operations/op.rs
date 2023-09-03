#[allow(unused)]
// +, -, 
// *, /, %
// &, |, ^, !,
// <<, >>

fn main () {
    println!("Addition:         {}", 2 + 5);
    println!{"Subtraction:      {}", 2 - 5};
    println!{"Multiplication:   {}", 2 * 5};
    println!("Division:         {}", 2 / 5);
    println!("Remainder:        {}", 2 % 5);


    // 0000 0000
    // decimal 1 = binary 0001
    // decimal 2 = binary 0010
    // decimal 3 = binary 0011
    // decimal 4 = binary 0100
    // decimal 5 = binary 0101

    let mut a: u8 = 5; // 0101
    let mut b: u8 = 3; // 0011

    println!("a & b = {}", a & b); // 0001
    println!("a | b = {}", a | b); // 0111
    println!("a ^ b = {}", a ^ b); // 0110
    println!("!a = {}, !b = {}", !a, !b); // 1110, 1100

    // 1     1   1   1  1  1  1  1
    // 128 +64 +32 +16 +8 +4 +2 +1 = 128*2 - 1 = 256 - 1 = 255

    println!("Max unsigned 8-bit int: {}", u8::MAX);

    let mut num: u8 = 1;
    println!("num: {}", num);

    for i in 1..10
    {
        num = num << 1; // Shift the bits to the right once
        println!("num: {}", num);  
    }

    println!("Now bitshift division");

    num = 128;
    println!("num: {}", num);

    for i in 1..10
    {
        // 0000 0000
        // 0000 0001
        // 0000 0010
        // 0000 0100
        // ...
        // 0010 0000
        // 0100 0000
        // 1000 0000
        num = num >> 1; // Shift the bits to the left once
        println!("num: {}", num);
    }
}

