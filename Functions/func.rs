// specified input type, specified output type
// explicit return
// non-explicit return

fn main() {
    println!("{}", adder(3,4));
    
    let  array: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

    println!("{}", average(&array));
}

fn adder(first_parameter: u32, second_parameter: u32) -> u32 {
    // This is an expression, the `return` keyword is not necessary here
    first_parameter + second_parameter
}

fn average(array: &[f32]) -> f32 {
    let mut average: f32 = 0.0;

    for ii in array
    {
        average += ii;
    }

    return average / array.len() as f32;
}