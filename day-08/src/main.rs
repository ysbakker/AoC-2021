use std::collections::HashMap;

fn main() {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let input = get_input::get_input_as_strings(INPUT);
    println!("Part 1: {}", count_easy_digits(&input));
    println!("Part 2: {}", calculate_output_value_sum(&input));
}

/// Returns Vec<(signal_patterns, output_digits)>
fn parse_input(input: &[String]) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut result: Vec<(Vec<&str>, Vec<&str>)> = Vec::new();
    for line in input {
        let inputs: Vec<&str> = line.split(" | ").collect();
        let signal_patterns: Vec<&str> = inputs[0].split_whitespace().collect();
        let output_digits: Vec<&str> = inputs[1].split_whitespace().collect();
        result.push((signal_patterns, output_digits))
    }
    result
}

fn count_easy_digits(input: &[String]) -> u32 {
    let parsed = parse_input(input);
    let mut count = 0;

    for (_pattern, output_values) in parsed {
        for number in output_values {
            if [2, 3, 4, 7].contains(&number.len()) {
                count += 1
            }
        }
    }

    count
}

fn calculate_output_value_sum(input: &[String]) -> u32 {
    let parsed = parse_input(input);
    let mut sum = 0;
    for entry in parsed {
        sum += calculate_output_value(entry);
    }
    sum
}

fn calculate_output_value(entry: (Vec<&str>, Vec<&str>)) -> u32 {
    let (patterns, output_values) = entry;
    let mut pattern_mapping: HashMap<u8, &str> = HashMap::new();

    while pattern_mapping.len() < 10 {
        for pattern in &patterns {
            if pattern.len() == 2 {
                pattern_mapping.insert(1, pattern);
                continue;
            } else if pattern.len() == 3 {
                pattern_mapping.insert(7, pattern);
                continue;
            } else if pattern.len() == 4 {
                pattern_mapping.insert(4, pattern);
                continue;
            } else if pattern.len() == 7 {
                pattern_mapping.insert(8, pattern);
                continue;
            }
            if pattern.len() == 5 {
                if pattern_mapping.get(&1).is_some()
                    && (pattern_mapping
                        .get(&1)
                        .unwrap()
                        .chars()
                        .all(|x| pattern.chars().any(|y| x == y)))
                {
                    pattern_mapping.insert(3, pattern);
                    continue;
                } else if pattern_mapping.get(&4).is_some()
                    && pattern_mapping.get(&1).is_some()
                    && (pattern_mapping
                        .get(&4)
                        .unwrap()
                        .chars()
                        .map(|x| pattern.chars().any(|y| x == y))
                        .filter(|x| x == &true)
                        .count()
                        == 3)
                {
                    pattern_mapping.insert(5, pattern);
                    continue;
                } else if pattern_mapping.get(&4).is_some() && pattern_mapping.get(&1).is_some() {
                    pattern_mapping.insert(2, pattern);
                    continue;
                }
            }
            if pattern.len() == 6 {
                if pattern_mapping.get(&4).is_some()
                    && (pattern_mapping
                        .get(&4)
                        .unwrap()
                        .chars()
                        .all(|x| pattern.chars().any(|y| x == y)))
                {
                    pattern_mapping.insert(9, pattern);
                    continue;
                } else if pattern_mapping.get(&4).is_some()
                    && pattern_mapping.get(&1).is_some()
                    && (pattern_mapping
                        .get(&1)
                        .unwrap()
                        .chars()
                        .all(|x| pattern.chars().any(|y| x == y)))
                {
                    pattern_mapping.insert(0, pattern);
                    continue;
                } else {
                    pattern_mapping.insert(6, pattern);
                    continue;
                }
            }
        }
    }
    let mut value: String = String::new();
    for output_value in output_values {
        for i in 0..10 {
            let pattern = pattern_mapping.get(&i).unwrap();
            if pattern.len() == output_value.len()
                && pattern
                    .chars()
                    .all(|x| output_value.chars().any(|y| x == y))
            {
                value += &i.to_string()[..];
                break;
            }
        }
    }

    value.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));

    #[test]
    fn test_parse_input() {
        let input = get_input::get_input_as_strings(INPUT);
        let parsed = parse_input(&input);
        assert_eq!(10, parsed.len());
        assert_eq!(10, parsed[0].0.len());
        assert_eq!(4, parsed[0].1.len());
    }
    #[test]
    fn test_count_easy_digits() {
        let input = get_input::get_input_as_strings(INPUT);
        assert_eq!(26, count_easy_digits(&input));
    }
    #[test]
    fn test_calculate_output_value_sum() {
        let input = get_input::get_input_as_strings(INPUT);
        assert_eq!(61229, calculate_output_value_sum(&input));
    }
}
