fn main() {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let fish: Vec<usize> = INPUT
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    println!("Part 1: {}", simulate_and_get_amount(&fish, 80));
    println!("Part 1: {}", simulate_and_get_amount(&fish, 256));
}

fn simulate_and_get_amount(fish: &[usize], iterations: u32) -> usize {
    let mut age_buckets: [usize; 9] = [0; 9];
    let mut next_age_buckets: [usize; 9] = [0; 9];
    for val in fish {
        age_buckets[*val] += 1
    }
    for _ in 0..iterations {
        for i in 0..9 {
            if i == 8 {
                next_age_buckets[i] = age_buckets[0]
            } else if i == 6 {
                next_age_buckets[i] = age_buckets[0] + age_buckets[i + 1]
            } else {
                next_age_buckets[i] = age_buckets[i + 1]
            }
        }
        age_buckets = next_age_buckets;
        next_age_buckets = [0; 9];
    }
    age_buckets.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));

    #[test]
    fn test_simulate_and_get_amount() {
        let fish: Vec<usize> = INPUT
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        assert_eq!(26, simulate_and_get_amount(&fish, 18));
        assert_eq!(5934, simulate_and_get_amount(&fish, 80));
        assert_eq!(26984457539, simulate_and_get_amount(&fish, 256));
    }
}
