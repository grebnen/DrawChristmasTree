#![allow(non_snake_case)]

use std::io::Write;
use std::time::Instant;

//This program takes gets an int input -h- from a user
// and draws a Christmas Tree with a height of h
fn main() {
    print!("Enter a positive integer for the height of a Christmas Tree: ");

    let height = get_user_int();

    //call function to draw tree or exit the app if the input is not a positive int
    if height <= 0 {
        println!("The height \'{}\' cannot be negative. Exiting application", height);
        std::process::exit(1);
    } else {
        draw_christmas_tree(height);
        draw_tree(height);
    }
}

//get user input as int value
pub fn get_user_int() -> i32 {
    let mut input_int = String::new();
    let _ = std::io::stdout().flush();
    match std::io::stdin().read_line(&mut input_int) {
        Ok(_) => () ,
        Err(e) => {
            println!("{}", e);
            return -1;
        }
    }

    match input_int.trim().parse::<i32>(){
        Ok(value) => value,
        Err(e) => {
            println!("Input was not an int");
            println!("Error message: {}", e);
            return -1;
        }
    }
}

//method to receive int input and print tree
pub fn draw_christmas_tree(h: i32) {
    let mut branches: i32;
    let mut spaces: i32;
    let loop_time = Instant::now();
    for index in 0..h {
        spaces = h - (index + 1);
        branches = (2 * index) + 1;
        for _ in 0..spaces {
            print!(" ");
        }

        for _ in 0..branches {
            print!("#");
        }

        if spaces == 0 {
            print!("\nCode Challenge Complete. Drawing time elapsed: {:.2?}", loop_time.elapsed());
            println!();
        } else {
            println!();
        }
    }
}

//method to attempt to print tree faster
pub fn draw_tree(h: i32){
    let mut branches: i32;
    let mut spaces: i32;
    let loop_time = Instant::now();

    //using one loop instead of 3 by using repeat() function
    //performance appears better when the amount of rows is lower
    for index in 0..h {
        spaces = h - (index + 1);
        branches = (2 * index) + 1;
        println!("{}{}", " ".repeat(spaces as usize), "#".repeat(branches as usize));
    }

    print!("\nCode Challenge Complete. Drawing time elapsed: {:.2?}", loop_time.elapsed());
}
