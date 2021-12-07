fn main() {
    let now = std::time::Instant::now();
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let input = get_input::get_input_from_binary_as_numbers(INPUT);
    println!("Part 1: {}", calculate_power_consumption(&input));
    println!("Part 2: {}", calculate_life_support_rating(&input));
    println!("Elapsed: {}ms", now.elapsed().as_millis()); // 2ms
}

fn calculate_power_consumption(numbers: &[u32]) -> u32 {
    let bits = get_max_bit_count(numbers);
    let gamma_rate = calculate_gamma_rate(numbers);
    (gamma_rate ^ (2u32.pow(bits) - 1)) * gamma_rate
}

fn calculate_gamma_rate(numbers: &[u32]) -> u32 {
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

fn calculate_life_support_rating(numbers: &[u32]) -> u32 {
    find_co2_scrubber_rating(numbers) * find_oxygen_generator_rating(numbers)
}

fn find_oxygen_generator_rating(numbers: &[u32]) -> u32 {
    let bits = get_max_bit_count(numbers);

    let mut current_numbers = numbers.to_vec();
    for i in 0..bits {
        let (zeros, ones) = split_numbers_by_index(&current_numbers, i, bits);
        if ones.len() >= zeros.len() {
            current_numbers = ones
        } else {
            current_numbers = zeros
        }

        if current_numbers.len() == 1 {
            return current_numbers[0];
        }
    }
    0
}

fn find_co2_scrubber_rating(numbers: &[u32]) -> u32 {
    let bits = get_max_bit_count(numbers);

    let mut current_numbers = numbers.to_vec();
    for i in 0..bits {
        let (zeros, ones) = split_numbers_by_index(&current_numbers, i, bits);
        if ones.len() < zeros.len() {
            current_numbers = ones
        } else {
            current_numbers = zeros
        }

        if current_numbers.len() == 1 {
            return current_numbers[0];
        }
    }
    0
}

/// returns `(zero_in_current_index, one_in_current_index)`
fn split_numbers_by_index(numbers: &[u32], index: u32, bits: u32) -> (Vec<u32>, Vec<u32>) {
    let compare = 1 << (bits - index - 1);
    let mut numbers_with_one_in_current_index = Vec::<u32>::new();
    let mut numbers_with_zero_in_current_index = Vec::<u32>::new();
    for number in numbers {
        if number & compare > 0 {
            numbers_with_one_in_current_index.push(*number)
        } else {
            numbers_with_zero_in_current_index.push(*number)
        }
    }

    (
        numbers_with_zero_in_current_index,
        numbers_with_one_in_current_index,
    )
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
    fn test_calculate_power_consumption() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(198, calculate_power_consumption(&input));
    }
    #[test]
    fn test_calculate_gamma_rate() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(22, calculate_gamma_rate(&input));
    }
    #[test]
    fn test_calculate_life_support_rating() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(230, calculate_life_support_rating(&input));
    }
    #[test]
    fn test_find_oxygen_generator_rating() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(23, find_oxygen_generator_rating(&input));
    }
    #[test]
    fn test_find_co2_scrubber_rating() {
        let input = get_input::get_input_from_binary_as_numbers(INPUT);
        assert_eq!(10, find_co2_scrubber_rating(&input));
    }
    #[test]
    fn test_get_max_bit_count() {
        assert_eq!(5, get_max_bit_count(&[30, 2, 5, 10]));
        assert_eq!(8, get_max_bit_count(&[30, 2, 128, 10]));
    }
}
