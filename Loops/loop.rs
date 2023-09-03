#[allow(unused)]
// Loop
// While
// For in
// Break, Continue 
// Nested Loops
// Video credits:
// https://nitter.c-r-t.tk/hackerrank/status/1611239886690091008#m
// https://www.tiktok.com/@brittanyraeannlong/video/7174253052344519978

use std::{thread, time};

fn main () {
    let array: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut average: f32 = 0.0;

    for ii in array
    {
        average += ii;
    }

    println!("Average: {}", average / array.len() as f32);
}