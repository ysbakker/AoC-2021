fn main() {
    let now = std::time::Instant::now();

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));

    let submarines: Vec<i32> = String::from(INPUT)
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    println!("Part 1: {}", calculate_minimum_fuel(&submarines));
    println!("Part 2: {}", calculate_minimum_fuel_part2(&submarines));
    println!("Elapsed: {}ms", now.elapsed().as_millis()); // ~109ms
}

fn calculate_minimum_fuel(submarine_positions: &[i32]) -> i32 {
    let mut fuel_usage: Vec<i32> = Vec::new();
    for submarine in submarine_positions {
        let mut usage = 0;
        for other_submarine in submarine_positions {
            let diff: i32 = submarine - other_submarine;
            usage += diff.abs()
        }
        fuel_usage.push(usage)
    }
    *fuel_usage.iter().min().unwrap()
}

fn calculate_fuel_part2(distance: i32) -> i32 {
    distance + (distance.pow(2) - distance) / 2
}

fn calculate_minimum_fuel_part2(submarine_positions: &[i32]) -> i32 {
    let mut fuel_usage: Vec<i32> = Vec::new();

    for i in 0..*submarine_positions.iter().max().unwrap() {
        let mut usage = 0;
        for sub in submarine_positions {
            let distance = sub - i;
            usage += calculate_fuel_part2(distance.abs());
        }
        fuel_usage.push(usage);
    }
    *fuel_usage.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));

    #[test]
    fn test_calculate_minimum_fuel() {
        let submarines: Vec<i32> = String::from(INPUT)
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        assert_eq!(37, calculate_minimum_fuel(&submarines));
    }

    #[test]
    fn test_calculate_minimum_fuel_part2() {
        let submarines: Vec<i32> = String::from(INPUT)
            .split(',')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        assert_eq!(168, calculate_minimum_fuel_part2(&submarines));
    }
    #[test]
    fn test_calculate_fuel_part2() {
        assert_eq!(66, calculate_fuel_part2(11));
        assert_eq!(10, calculate_fuel_part2(4));
        assert_eq!(6, calculate_fuel_part2(3));
        assert_eq!(1, calculate_fuel_part2(1));
    }
}
