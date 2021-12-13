fn main() {
    let now = std::time::Instant::now();

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));

    let lines = get_input::get_input_as_strings(INPUT);
    println!("Part 1: {}", calculate_syntax_error_score(&lines));
    println!("Part 2: {}", calculate_autocomplete_score(&lines));
    println!("Elapsed: {}ms", now.elapsed().as_millis()); // 3ms
}

fn calculate_syntax_error_score(lines: &[String]) -> u32 {
    let mut score = 0;
    for line in lines {
        let c = parse_chunks(line);
        if c.is_err() {
            score += map_character_to_syntax_error_score(c.err().unwrap());
        }
    }
    score
}

fn calculate_autocomplete_score(lines: &[String]) -> u64 {
    let mut scores: Vec<u64> = Vec::new();
    let remaining_lines = lines.iter().map(|x| parse_chunks(x)).filter(|x| x.is_ok());
    for line in remaining_lines {
        let completion_string: String = line
            .unwrap()
            .iter()
            .rev()
            .map(|x| get_corresponding_closing_tag(*x))
            .collect();
        let mut score: u64 = 0;
        for c in completion_string.chars() {
            score *= 5;
            score += map_character_to_autocomplete_score(c) as u64;
        }
        scores.push(score)
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse_chunks(line: &str) -> Result<Vec<char>, char> {
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
                return Err(c);
            }
        }
    }
    Ok(chunk_stack)
}

fn map_character_to_syntax_error_score(character: char) -> u32 {
    match character {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn map_character_to_autocomplete_score(character: char) -> u32 {
    match character {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));

    #[test]
    fn test_calculate_syntax_error_score() {
        let lines = get_input::get_input_as_strings(INPUT);
        assert_eq!(26397, calculate_syntax_error_score(&lines))
    }
    #[test]
    fn test_calculate_autocomplete_score() {
        let lines = get_input::get_input_as_strings(INPUT);
        assert_eq!(288957, calculate_autocomplete_score(&lines))
    }
}
