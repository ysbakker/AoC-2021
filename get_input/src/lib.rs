pub fn get_input_as_numbers(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

pub fn get_input_as_strings(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect::<Vec<String>>()
}

pub fn get_input_from_binary_as_numbers(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap())
        .collect()
}
