pub fn get_input_as_numbers(input: &str) -> Vec<i16> {
    return input.lines().map(|x| x.parse::<i16>().unwrap()).collect();
}

pub fn get_input_as_strings(input: &str) -> Vec<String> {
    let split = input.lines().map(|x| String::from(x));
    return split.collect::<Vec<String>>();
}
