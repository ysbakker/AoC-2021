fn main() {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let input = get_input::get_input_from_binary_as_numbers(INPUT);
    println!(
        "Part 1: {}",
        find_gamma_rate(&input) * find_epsilon_rate(&input)
    )
}

fn find_most_common_bits(numbers: &[u32]) -> u32 {
    let bits = get_max_bit_count(numbers);
    let mut compare = 1;
    let mut result = 0;
    for i in 0..bits {
        let mut amount_of_ones = 0;
        for number in numbers {
            if number & compare > 0 {
                amount_of_ones += 1;
            }
        }

        if amount_of_ones > numbers.len() / 2 {
            result += 2u32.pow(i);
        }
        compare <<= 1
    }
    result
}

fn find_gamma_rate(numbers: &[u32]) -> u32 {
    find_most_common_bits(numbers)
}

fn find_epsilon_rate(numbers: &[u32]) -> u32 {
    let bits = get_max_bit_count(numbers);
    let most_common = find_most_common_bits(numbers);
    most_common ^ (2u32.pow(bits) - 1)
}

fn find_oxygen_generator_rating(numbers: &[u32]) -> u32 {
    0
}

/// Finds the bit count of the largest value in a collection of u32.
///
/// Examples:
/// - `&[2,30,10,11]` -> 30 -> 11110 -> 5
/// - `&[6,9,130,40]` -> 130 -> 10000010 -> 8
fn get_max_bit_count(numbers: &[u32]) -> u32 {
    let max = numbers.iter().max().unwrap();
    (std::mem::size_of::<i32>() * 8) as u32 - max.leading_zeros()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));
    #[test]
    fn test_power_consumption() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(198, find_epsilon_rate(&input) * find_gamma_rate(&input));
    }

    #[test]
    fn test_find_gamma_rate() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(22, find_gamma_rate(&input));
    }
    #[test]
    fn test_find_epsilon_rate() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(9, find_epsilon_rate(&input));
    }
    #[test]
    fn test_find_oxygen_generator_rating() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(9, find_epsilon_rate(&input));
    }
    #[test]
    fn test_get_max_bit_count() {
        assert_eq!(5, get_max_bit_count(&[30, 2, 5, 10]));
        assert_eq!(8, get_max_bit_count(&[30, 2, 128, 10]));
    }
}
