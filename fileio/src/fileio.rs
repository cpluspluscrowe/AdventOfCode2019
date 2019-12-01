use std::fs::File;
use std::io::{BufRead, BufReader};

mod fileio;

fn main() {
    println!("Hello World");
}

pub fn get_single_number_input(path: &str) -> Vec<i32> {
    let mut vec = Vec::new();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let number: i32 = line.unwrap().parse().unwrap();
        vec.push(number);
    }
    vec
}

pub fn get_array_of_numbers_input(path: &str) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::new();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {

        let numbers: Vec<i32> = line.unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(numbers);
    }
    vec
}

#[test]
fn simple() {
    assert_eq!(1, 1);
}

#[test]
fn test_get_single_numbers_input() {
    let numbers = get_single_number_input("src/input.txt");
    let mut expected = vec![1, 2, 3, 4, 5];
    assert_eq!(numbers, expected);
}

#[test]
fn test_get_array_of_numbers_input() {
    let numbers = get_array_of_numbers_input("src/array_input.txt");
    let v1 = vec![1,2,3];
    let v2 = vec![4,5,6];
    assert_eq!(numbers[0], v1);
    assert_eq!(numbers[1], v2);
}
