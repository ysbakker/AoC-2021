use get_input;

fn main() {
    let input = get_input::get_input_as_strings("./input/day-02/input");
    let commands = parse_commands(&input);
    println!("Part 1: {}", move_sub_and_get_product(&commands));
    println!("Part 2: {}", move_sub_and_get_product_with_aim(&commands));
}

/**
 * returns (x_pos, depth)
 */
fn calculate_new_position(mut x_pos: i32, mut depth: i32, command: &(&str, u16)) -> (i32, i32) {
    match command.0 {
        "up" => depth -= command.1 as i32,
        "down" => depth += command.1 as i32,
        "forward" => x_pos += command.1 as i32,
        _ => (),
    }

    return (x_pos, depth);
}

/**
 * returns (x_pos, depth, aim)
 */
fn calculate_new_position_with_aim(
    mut x_pos: i32,
    mut depth: i32,
    mut aim: i32,
    command: &(&str, u16),
) -> (i32, i32, i32) {
    match command.0 {
        "up" => aim -= command.1 as i32,
        "down" => aim += command.1 as i32,
        "forward" => {
            x_pos += command.1 as i32;
            depth += aim * command.1 as i32;
        }
        _ => (),
    }

    return (x_pos, depth, aim);
}

fn move_sub_and_get_product(commands: &Vec<(&str, u16)>) -> i32 {
    let mut x_pos = 0;
    let mut depth = 0;
    for command in commands {
        let new_pos = calculate_new_position(x_pos, depth, command);
        x_pos = new_pos.0;
        depth = new_pos.1;
    }

    return x_pos * depth;
}

fn move_sub_and_get_product_with_aim(commands: &Vec<(&str, u16)>) -> i32 {
    let mut x_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in commands {
        let new_pos = calculate_new_position_with_aim(x_pos, depth, aim, command);
        x_pos = new_pos.0;
        depth = new_pos.1;
        aim = new_pos.2;
    }

    return x_pos * depth;
}

/**
 * returns Vec<(command, amount)>
 */
fn parse_commands(commands: &Vec<String>) -> Vec<(&str, u16)> {
    let mut parsed = Vec::<(&str, u16)>::new();
    for command in commands {
        let mut split = command.split(" ");
        parsed.push((
            split.next().unwrap(),
            split.next().unwrap().parse::<u16>().unwrap(),
        ));
    }
    return parsed;
}

#[test]
fn test_move_sub() {
    let input = get_input::get_input_as_strings("../input/day-02/test");
    let commands = parse_commands(&input);
    assert_eq!(150, move_sub_and_get_product(&commands))
}

#[test]
fn test_move_sub_with_aim() {
    let input = get_input::get_input_as_strings("../input/day-02/test");
    let commands = parse_commands(&input);
    assert_eq!(900, move_sub_and_get_product_with_aim(&commands))
}
