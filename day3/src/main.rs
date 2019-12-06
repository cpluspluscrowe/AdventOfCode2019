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

fn replace_direction_with_tuple(vectors: Vec<Vec<(String, i32)>>) -> Vec<Vec<((i32, i32), i32)>> {
    vectors.iter().map(|x| x.iter().map(|y| (get_direction_indexes(&y.0), y.1)).collect()).collect()
}

fn fill_array(direction_magnitudes: Vec<Vec<((i32, i32), i32)>>, mut toFill: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let vector_size = toFill.len();
    for path_vectors in direction_magnitudes {
        let mut row_index = vector_size - 1;
        let mut column_index = 0;
        for vector in path_vectors {
            let (vertical_component, horizontal_component) = vector.0;
            let magnitude = vector.1;
            for interval in 0..magnitude {
                row_index = row_index + (vertical_component as usize);
                column_index = column_index + (horizontal_component as usize);
                toFill[row_index][column_index] += 1
            }
        }
    }
     toFill
}

fn main() {
    let path = "src/input.txt";
    let input = get_array_of_string_input(path);
    let vectors: Vec<Vec<(String, i32)>> = input
        .iter()
        .map(|x| operate_on_vector(x.to_vec()))
        .collect();
    let direction_magnitudes = replace_direction_with_tuple(vectors.to_vec());
    // Determines maximum 2D array size
    let summation = get_magnitude_summation(vectors);
    let mut array = vec![vec![0; summation as usize]; summation as usize];
    //    let mut array_to_fill  = [[0, summation]; summation];
    //    println!("{}",summation);
    let filled = fill_array(direction_magnitudes, array);
    println!("Hello, world!");
}
