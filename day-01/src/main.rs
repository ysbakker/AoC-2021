use get_input;

fn main() {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/input"));
    let data = get_input::get_input_as_numbers(INPUT);
    println!("Part 1: {}", get_depth_increases(&data));
    println!("Part 2: {}", get_sliding_window_increases(&data));
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
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));
    let data = get_input::get_input_as_numbers(INPUT);
    assert_eq!(7, get_depth_increases(&data));
}

#[test]
fn test_slidingwindowincreases() {
    const INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/test"));
    let data = get_input::get_input_as_numbers(INPUT);
    assert_eq!(5, get_sliding_window_increases(&data));
}
