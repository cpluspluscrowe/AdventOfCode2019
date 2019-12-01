use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let path = "src/example_input.txt";
    let summation = get_mass_summation(path);
    println!("The summation = {}", summation);
    Ok(())
}

fn get_mass_summation(path: &str) -> i32 {
    let lines = get_file_lines(path);
    let mut summation: i32 = 0;
    for line in lines {
        let line: i32 = line.unwrap().parse().unwrap();
        summation += calculate_fuel_required(line);
    }
    summation
}

fn get_file_lines(path: &str) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    buffered.lines()
}

fn calculate_fuel_required(x: i32) -> i32 {
    x / 3 - 2
}

#[test]
fn example1() {
    assert_eq!(calculate_fuel_required(14), 2);
}

#[test]
fn example2() {
    assert_eq!(calculate_fuel_required(12), 2);
}

#[test]
fn example3() {
    assert_eq!(calculate_fuel_required(1969), 654);
}

#[test]
fn example4() {
    assert_eq!(calculate_fuel_required(100756), 33583);
}

#[test]
fn summation() {
    let path = "src/example_input.txt";
    assert_eq!(get_mass_summation(path), 34241);
}
