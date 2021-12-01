use std::fs;

fn main() {
    let data = get_input("./data/input");
    println!("Part 1: {}", get_depth_increases(&data));
    println!("Part 2: {}", get_sliding_window_increases(&data));
}

fn get_input(file_path: &str) -> Vec<i16> {
    let data = fs::read_to_string(file_path).unwrap();
    return data
        .split("\n")
        .map(|x| x.parse::<i16>().unwrap())
        .collect();
}

fn get_depth_increases(data: &Vec<i16>) -> i16 {
    let mut increases: i16 = 0;
    let mut prev: i16 = data[0];

    for val in data {
        if val > &prev {
            increases += 1;
        }
        prev = *val;
    }

    return increases;
}

fn get_sliding_window_increases(data: &Vec<i16>) -> i16 {
    let mut increases: i16 = 0;
    let mut prev: i16 = data[0] + data[1] + data[2];

    for (i, val) in data.iter().enumerate() {
        if i + 3 > data.len() {
            break;
        }
        let sum = val + data[i + 1] + data[i + 2];
        if sum > prev {
            increases += 1;
        }
        prev = sum;
    }

    return increases;
}

#[test]
fn test_depthincreases() {
    let data = get_input("./data/test");
    assert_eq!(7, get_depth_increases(&data));
}

#[test]
fn test_slidingwindowincreases() {
    let data = get_input("./data/test");
    assert_eq!(5, get_sliding_window_increases(&data));
}
