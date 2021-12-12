fn main() {
    let now = std::time::Instant::now();

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let lines = get_input::get_input_as_strings(INPUT);
    let matrix = parse_input_to_matrix(&lines);
    let low_points = find_low_points(&matrix);
    println!("Part 1: {}", calculate_risk_level_sum(&low_points));
    println!("Elapsed: {}ms", now.elapsed().as_millis()); // 2ms
}

fn parse_input_to_matrix(lines: &[String]) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

        matrix.push(row)
    }

    matrix
}

fn find_low_points(matrix: &[Vec<u32>]) -> Vec<u32> {
    let mut low_points: Vec<u32> = Vec::new();

    for (x, row) in matrix.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            let mut is_low_point = true;
            for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if x as i32 + d.0 < 0 || y as i32 + d.1 < 0 {
                    continue;
                }
                let crow = matrix.get((x as i32 + d.0) as usize);
                if let Some(..) = crow {
                    let ccol = crow.unwrap().get((y as i32 + d.1) as usize);
                    if ccol.is_some() && ccol.unwrap() <= col {
                        is_low_point = false;
                        break;
                    }
                }
            }
            if is_low_point {
                low_points.push(*col);
            }
        }
    }

    low_points
}

fn calculate_risk_level_sum(low_points: &[u32]) -> u32 {
    low_points.iter().map(|x| x + 1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));

    #[test]
    fn test_find_low_points() {
        let lines = get_input::get_input_as_strings(INPUT);
        let matrix = parse_input_to_matrix(&lines);
        let low_points = find_low_points(&matrix);
        println!("{:?}", low_points);
        assert_eq!(4, low_points.len());
        assert_eq!(11_u32, low_points.iter().sum());
    }
    #[test]
    fn test_calculate_risk_level_sum() {
        assert_eq!(15, calculate_risk_level_sum(&[1, 0, 5, 5]));
    }
}
