use std::cmp::max;

fn main() {
    let now = std::time::Instant::now();
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let lines = get_input::get_input_as_strings(INPUT);
    let grid = parse_grid(&lines);
    println!("Part 1: {}", model_steps(&grid, 100));
    println!("Part 2: {}", find_first_synchronous_step(&grid));
    println!("Elapsed: {}ms", now.elapsed().as_millis()); // 64ms
}

fn find_first_synchronous_step(grid: &[Vec<i32>]) -> u32 {
    let mut grid_copy = grid.to_owned();
    let max_flashes = (grid.len() * grid[0].len()) as u32;
    let mut current_step = 0;
    loop {
        current_step += 1;
        let flashes = model_step(&mut grid_copy);
        if flashes == max_flashes {
            break;
        }
    }
    current_step
}

fn model_steps(grid: &[Vec<i32>], steps: u32) -> u32 {
    let mut grid_copy = grid.to_owned();
    let mut total_flashes = 0;
    for _ in 0..steps {
        total_flashes += model_step(&mut grid_copy);
    }
    total_flashes
}

fn model_step(grid: &mut [Vec<i32>]) -> u32 {
    let mut change = true;
    let mut first_pass = true;
    while change {
        change = false;
        for (y, row) in grid.to_owned().iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if first_pass {
                    grid[y][x] = (max(col, &0) + 1) % 10;
                    change = true;
                    continue;
                }
                if col == &0 {
                    for (dy, dx) in [
                        (-1, -1),
                        (-1, 0),
                        (-1, 1),
                        (0, -1),
                        (0, 1),
                        (1, -1),
                        (1, 0),
                        (1, 1),
                    ] {
                        let (cy, cx) = (y as i32 + dy, x as i32 + dx);
                        if cy < 0 || cy >= grid.len() as i32 || cx < 0 || cx >= row.len() as i32 {
                            continue;
                        }

                        let cur = grid[cy as usize][cx as usize];

                        if ![0, -1].contains(&cur) {
                            change = true;
                            grid[cy as usize][cx as usize] = (cur + 1) % 10;
                        }
                    }
                    grid[y][x] = -1;
                }
            }
        }
        if first_pass {
            first_pass = false;
        }
    }
    grid.iter()
        .map(|row| {
            return row
                .iter()
                .fold(0_u32, |acc, c| if c == &-1 { acc + 1 } else { acc });
        })
        .sum()
}

fn parse_grid(lines: &[String]) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect(),
        )
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));

    #[test]
    fn test_model_step() {
        let input = get_input::get_input_as_strings(INPUT);
        let mut grid = parse_grid(&input);
        assert_eq!(0, model_step(&mut grid));
        grid = parse_grid(&[
            String::from("11111"),
            String::from("19991"),
            String::from("19191"),
            String::from("19991"),
            String::from("11111"),
        ]);
        assert_eq!(9, model_step(&mut grid));
    }

    #[test]
    fn test_model_steps() {
        let input = get_input::get_input_as_strings(INPUT);
        let grid = parse_grid(&input);
        assert_eq!(204, model_steps(&grid, 10));
        assert_eq!(1656, model_steps(&grid, 100));
    }

    #[test]
    fn test_find_first_synchronous_step() {
        let input = get_input::get_input_as_strings(INPUT);
        let grid = parse_grid(&input);
        assert_eq!(195, find_first_synchronous_step(&grid));
    }
}
