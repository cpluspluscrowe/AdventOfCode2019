use fileio::get_array_of_string_input;

pub fn operate_on_vector(vector: Vec<String>) -> Vec<(String, i32)> {
    vector
        .iter()
        .map(|x| separate_direction_from_magnitude(x.to_string()))
        .collect()
}

pub fn separate_direction_from_magnitude(vector: String) -> (String, i32) {
    let (first, last) = vector.split_at(1);
    (first.to_string(), last.to_string().parse().unwrap())
}

pub fn get_magnitude_summation(vector: Vec<Vec<(String, i32)>>) -> i32 {
    vector
        .iter()
        .map(|x| {
            ((x.iter().map(|y| y.1).collect::<Vec<i32>>())
                .iter()
                .sum::<i32>())
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

pub fn get_direction_indexes(direction: &str) -> (i32, i32) {
    match direction {
        "L" => (0, -1),
        "U" => (-1, 0),
        "R" => (0, 1),
        "D" => (1, 0),
        _ => panic!("direction not matched"),
    }
}

fn replace_direction_with_tuple(vectors: Vec<Vec<(i32, String)>>) -> Vec<Vec<(i32, (i32, i32))>> {
    vectors.iter().map(|x| x.iter().map(|y| (y.0, get_direction_indexes(&y.1))).collect()).collect()
}

fn main() {
    let path = "src/input.txt";
    let input = get_array_of_string_input(path);
    let vectors: Vec<Vec<(String, i32)>> = input
        .iter()
        .map(|x| operate_on_vector(x.to_vec()))
        .collect();
    // Determines maximum 2D array size
    let summation = get_magnitude_summation(vectors);
    let mut array = vec![vec![0; summation as usize]; summation as usize];
    //    let mut array_to_fill  = [[0, summation]; summation];
    //    println!("{}",summation);
    println!("Hello, world!");
}
