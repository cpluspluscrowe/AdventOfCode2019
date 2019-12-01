//use std::collections::LinkedList;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello World");
}

fn get_single_number_input(path: &str) -> Vec<i32> {
    let mut vec = Vec::new();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let number: i32 = line.unwrap().parse().unwrap();
        vec.push(number);
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
