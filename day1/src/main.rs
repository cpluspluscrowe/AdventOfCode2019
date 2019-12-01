use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::cmp;

fn main() -> Result<(), Error> {
    let path = "src/puzzle_input.txt";
    let old = get_mass_summation(path);
    let summation = get_resursive_mass_summation(path);
    println!("The part1 solution is = {}", old);
    println!("The part2 solution = {}", summation);
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

fn get_resursive_mass_summation(path: &str) -> i32 {
    let lines = get_file_lines(path);
    let mut summation: i32 = 0;
    for line in lines {
        let line: i32 = line.unwrap().parse().unwrap();
        summation += calculate_recursive_fuel_required(line);
    }
    summation
}

fn get_file_lines(path: &str) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    buffered.lines()
}

fn calculate_recursive_fuel_required(x: i32) -> i32 {
    if x <= 0 {
        return 0;
    } else {
        let mass = cmp::max(calculate_fuel_required(x), 0);
        return mass + calculate_recursive_fuel_required(mass);
    }
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

#[test]
fn recursive_summation_example1() {
    let _path = "src/example_input.txt";
    assert_eq!(calculate_recursive_fuel_required(14), 2);
}

#[test]
fn recursive_summation_example2() {
    let _path = "src/example_input.txt";
    assert_eq!(calculate_recursive_fuel_required(1969), 966);
}

#[test]
fn recursive_summation_example3() {
    let _path = "src/example_input.txt";
    assert_eq!(calculate_recursive_fuel_required(100756), 50346);
}
