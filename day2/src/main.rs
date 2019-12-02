use std::io::{Error};
use fileio::get_array_of_numbers_input;

fn main() -> Result<(), Error> {
    let path = "src/example_input.txt";
    let lines = get_array_of_numbers_input(path);
    let input = lines[0].to_vec();
    let output = run_updates(input, 0, 1, 2, 3);
    for value in output {
        println!("{}", value);
    }
    println!("Hello World");
    Ok(())
}

pub fn run_updates(mut input: Vec<i32>, index0: usize, index1: usize, index2: usize, index3: usize) -> Vec<i32> {
    let value0 = input[index0];
    if value0 == 99 {
        return input;
    }
    let value1 = input[index1];
    let value2 = input[index2];
    let addition = value1 + value2;
    let destination: usize = input[index3] as usize;
    input[destination] = addition;
    run_updates(input, index0 + 4, index1 + 4, index2 + 4, index3 + 4)
}



/*pub fn print_data(input: &Vec<i32>){
    for value in input {
        println!("{}",value);
    }
}*/

