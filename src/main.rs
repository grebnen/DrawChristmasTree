#![allow(non_snake_case)]

use std::io::Write;

//This program takes gets an int input -h- from a user
// and draws a Christmas Tree with a height of h
fn main() {
    print!("Enter an integer for the height of a Christmas Tree: ");
    println!("{}", get_user_int());
}

pub fn get_user_int() -> i32 {
    let mut input_int = String::new();
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut input_int)
        .expect("Failed to enter an integer.");

    let height: i32 = input_int
        .trim()
        .parse()
        .expect("Input is not an integer");

    return height;
}
