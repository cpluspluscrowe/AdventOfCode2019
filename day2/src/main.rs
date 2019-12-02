use std::io::{Error};
use fileio::get_array_of_numbers_input;

fn main() -> Result<(), Error> {
    let path = "src/example_input.txt";
    let lines = get_array_of_numbers_input(path);
    let input = lines[0].to_vec();
    run_updates(input);
    println!("Hello World");
    Ok(())
}

pub fn run_updates(mut input: Vec<i32>) -> Vec<i32> {
    input[0] = 5;
    input
}



/*pub fn print_data(input: &Vec<i32>){
    for value in input {
        println!("{}",value);
    }
}*/

