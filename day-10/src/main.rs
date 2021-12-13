fn main() {
    let now = std::time::Instant::now();

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));

    let lines = get_input::get_input_as_strings(INPUT);
    println!("Part 1: {}", calculate_syntax_error_score(&lines));
    println!("Elapsed: {}ms", now.elapsed().as_millis()); // 1ms
}

fn map_character_to_score(character: char) -> u32 {
    match character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_corresponding_closing_tag(opening_tag: char) -> char {
    match opening_tag {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => '_',
    }
}

fn find_first_illegal_character(line: &str) -> Option<char> {
    let mut chunk_stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if ['(', '[', '{', '<'].contains(&c) {
            chunk_stack.push(c);
        } else if [')', ']', '}', '>'].contains(&c) {
            if !chunk_stack.is_empty()
                && c == get_corresponding_closing_tag(chunk_stack.last().copied().unwrap())
            {
                chunk_stack.pop();
            } else {
                return Some(c);
            }
        }
    }
    None
}

fn calculate_syntax_error_score(lines: &[String]) -> u32 {
    let mut score = 0;
    for line in lines {
        let c = find_first_illegal_character(line);
        if let Some(..) = c {
            score += map_character_to_score(c.unwrap());
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));

    #[test]
    fn test_calculate_syntax_error_score() {
        let lines = get_input::get_input_as_strings(INPUT);
        assert_eq!(26397, calculate_syntax_error_score(&lines))
    }
}
