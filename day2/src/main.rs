use std::io::{Error};
use fileio::get_array_of_numbers_input;

fn main() -> Result<(), Error> {
    let path = "src/example_input.txt";
    let lines = get_array_of_numbers_input(path);
    let input = &lines[0];
    print_data(input);
    println!("Hello World");
    Ok(())
}

pub fn print_data(input: &Vec<i32>){
    for value in input {
        println!("{}",value);
    }
}

