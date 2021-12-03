use std::fs;

pub fn get_input_as_numbers(file_path: &str) -> Vec<i16> {
    let data = fs::read_to_string(file_path).unwrap();
    return data
        .split("\n")
        .map(|x| x.parse::<i16>().unwrap())
        .collect();
}

pub fn get_input_as_strings(file_path: &str) -> Vec<String> {
    let data = fs::read_to_string(file_path).unwrap();
    let split = data.split("\n").map(|x| String::from(x));
    return split.collect::<Vec<String>>();
}
