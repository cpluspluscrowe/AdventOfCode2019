use std::fs::File;
use std::io::{BufRead, BufReader};
//use regex::Regex;

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
        let numbers: Vec<i32> = line
            .unwrap()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        vec.push(numbers);
    }
    vec
}

pub fn get_array_of_string_input(path: &str) -> Vec<Vec<String>> {
    let mut vec  = Vec::new();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let numbers: Vec<String> = line
            .unwrap().split(",").map(|s| s.to_string()).collect();
        vec.push(numbers);
    }
    vec
}

pub fn get_input_lines(path: &str) -> Vec<String> {
    let mut vec = Vec::new();
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let line: String = line.unwrap();
        vec.push(line);
    }
    vec
}

/*pub fn apply_regex_to_input(text: &str,regular_expression: &str) -> regex::re_unicode::Captures {
    let re = Regex::new(regular_expression).unwrap();
    re.captures(text).unwrap()
}*/

#[test]
fn test_get_single_number_input() {
    let numbers = get_single_number_input("src/input.txt");
    let expected = vec![1, 2, 3, 4, 5];
    assert_eq!(numbers, expected);
}

#[test]
fn test_get_input_lines() {
    let numbers = get_single_number_input("src/input.txt");
    let expected = vec![1, 2, 3, 4, 5];
    assert_eq!(numbers, expected);
}

#[test]
fn test_get_array_of_string_input() {
    let numbers: Vec<Vec<String>> = get_array_of_string_input("src/array_string_input.txt");
    let v1: Vec<String> = "U7,R6,D4,L4".split(",").map(String::from).collect();
        let v2: Vec<String> = "U7,R6,D4,L4".split(",").map(String::from).collect();
    assert_eq!(numbers[0], v1);
    assert_eq!(numbers[1], v2);
}
