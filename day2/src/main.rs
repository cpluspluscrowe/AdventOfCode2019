use fileio::get_array_of_numbers_input;
use std::io::Error;

// Day1, part 1 answer: 5434663

fn main() -> Result<(), Error> {
    let path = "src/puzzle_input.txt";
    let input = get_data(path);
    let output = update_values(input, 0, 1, 2, 3);
    for value in output {
        println!("{}", value);
    }
    println!("Hello World");
    Ok(())
}

pub fn get_data(path: &str) -> Vec<i32> {
    let lines = get_array_of_numbers_input(path);
    let input = lines[0].to_vec();
    input.to_vec()
}

pub fn update_values(
    mut input: Vec<i32>,
    index0: usize,
    index1: usize,
    index2: usize,
    index3: usize,
) -> Vec<i32> {
    let value0 = input[index0];
    if value0 == 99 {
        return input;
    }
    let value1 = input[input[index1] as usize];
    let value2 = input[input[index2] as usize];
    let destination: usize = input[index3] as usize;
    let addition = value1 + value2;
    let multiplication = value1 * value2;
    if value0 == 1 {
        input[destination] = addition;
    } else if value0 == 2 {
        input[destination] = multiplication;
    } else {
        panic!("value0 should only ever be equal to 1 or 2");
    }
    update_values(input, index0 + 4, index1 + 4, index2 + 4, index3 + 4)
}

#[test]
fn example1() {
    let _path = "src/example_input.txt";
    let input = get_data(_path);
    let output = update_values(input, 0, 1, 2, 3);
    let expected = vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

    assert_eq!(output, expected);
}

#[test]
fn example2() {
    let _path = "src/example2_input.txt";
    let input = get_data(_path);
    let output = update_values(input, 0, 1, 2, 3);
    let expected = vec![2,0,0,0,99];

    assert_eq!(output, expected);
}

#[test]
fn example3() {
    let _path = "src/example3_input.txt";
    let input = get_data(_path);
    let output = update_values(input, 0, 1, 2, 3);
    let expected = vec![2,3,0,6,99];

    assert_eq!(output, expected);
}

#[test]
fn example4() {
    let _path = "src/example4_input.txt";
    let input = get_data(_path);
    let output = update_values(input, 0, 1, 2, 3);
    let expected = vec![2,4,4,5,99,9801];

    assert_eq!(output, expected);
}

#[test]
fn example5() {
    let _path = "src/example4_input.txt";
    let input = get_data(_path);
    let output = update_values(input, 0, 1, 2, 3);
    let expected = vec![2,4,4,5,99,9801];

    assert_eq!(output, expected);
}
