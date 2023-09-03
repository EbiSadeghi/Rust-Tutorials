// if, else
// ternary operator
// match , ..u8::MAX, std::cmp::ordering
use std::cmp::Ordering;

fn main() {
    let my_height_in_cm = 180;
    let my_friends_height_in_cm = 180;

    match my_friends_height_in_cm.cmp(&my_height_in_cm)
    {
        Ordering::Less      => println! ("Shorter than me"),
        Ordering::Equal     => println! ("Same height as me"),
        Ordering::Greater   => println! ("Taller than me"),
    }
}
