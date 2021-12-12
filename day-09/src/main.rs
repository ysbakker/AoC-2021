use std::collections::HashSet;

fn main() {
    let now = std::time::Instant::now();

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let lines = get_input::get_input_as_strings(INPUT);
    let matrix = parse_input_to_matrix(&lines);
    let low_points = find_low_points(&matrix);
    let basin_sizes = find_basin_sizes(&matrix);
    println!("Part 1: {}", calculate_risk_level_sum(&low_points));
    println!("Part 2: {}", calculate_largest_basin_product(&basin_sizes));
    println!("Elapsed: {}ms", now.elapsed().as_millis()); // 28ms
}

fn parse_input_to_matrix(lines: &[String]) -> Vec<Vec<u32>> {
    let mut matrix: Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();

        matrix.push(row)
    }

    matrix
}

fn find_low_points_indexes(matrix: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut low_points_indexes: Vec<(usize, usize)> = Vec::new();

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
                low_points_indexes.push((x, y));
            }
        }
    }

    low_points_indexes
}

fn find_low_points(matrix: &[Vec<u32>]) -> Vec<u32> {
    let indexes = find_low_points_indexes(matrix);
    indexes.iter().map(|val| matrix[val.0][val.1]).collect()
}

fn calculate_risk_level_sum(low_points: &[u32]) -> u32 {
    low_points.iter().map(|x| x + 1).sum()
}

fn find_basin_sizes(matrix: &[Vec<u32>]) -> Vec<usize> {
    let mut basin_sizes: Vec<usize> = Vec::new();
    let low_point_indexes = find_low_points_indexes(matrix);

    for low_point_index in low_point_indexes {
        let mut basin_coords = HashSet::from([low_point_index]);
        let mut search_coords = Vec::from([low_point_index]);
        while !search_coords.is_empty() {
            let current_search = search_coords.clone();
            search_coords = Vec::new();
            for (x, y) in current_search {
                for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let (cur_x, cur_y) = (x as i32 + dx, y as i32 + dy);
                    if cur_x < 0 || cur_y < 0 {
                        continue;
                    }
                    let crow = matrix.get(cur_x as usize);
                    if let Some(..) = crow {
                        let ccol = crow.unwrap().get(cur_y as usize);
                        if ccol.is_some()
                            && ccol.unwrap() != &9
                            && !basin_coords.contains(&(cur_x as usize, cur_y as usize))
                        {
                            search_coords.push((cur_x as usize, cur_y as usize));
                            basin_coords.insert((cur_x as usize, cur_y as usize));
                        }
                    }
                }
            }
        }
        basin_sizes.push(basin_coords.len());
    }

    basin_sizes
}

fn calculate_largest_basin_product(basin_sizes: &[usize]) -> usize {
    let mut basin_sizes_clone = basin_sizes.to_owned();
    basin_sizes_clone.sort_unstable();
    basin_sizes_clone.iter().rev().take(3).product()
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
    #[test]
    fn test_find_basin_sizes() {
        let lines = get_input::get_input_as_strings(INPUT);
        let matrix = parse_input_to_matrix(&lines);
        let basin_sizes = find_basin_sizes(&matrix);
        println!("{:?}", basin_sizes);
        assert_eq!(4, basin_sizes.len());
        assert!(basin_sizes.iter().all(|x| [3, 9, 14].contains(x)));
    }
    #[test]
    fn test_calculate_largest_basin_product() {
        let lines = get_input::get_input_as_strings(INPUT);
        let matrix = parse_input_to_matrix(&lines);
        let basin_sizes = find_basin_sizes(&matrix);
        println!("{:?}", basin_sizes);
        assert_eq!(1134, calculate_largest_basin_product(&basin_sizes));
    }
}
