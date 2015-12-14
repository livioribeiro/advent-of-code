use std::convert::From;

const INPUT: &'static str = include_str!("data/day6.txt");

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

pub fn part1() -> u32 {
    calculate_part1(INPUT)
}

pub fn part2() -> u32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u32 {
    let mut lights = [false; WIDTH * HEIGHT];

    for line in input.lines() {
        let instruction = Instruction::from(line);
        change_lights_part1(instruction, &mut lights);
    }

    lights.iter().fold(0, |acc, light| if *light { acc + 1 } else { acc })
}

fn change_lights_part1(instruction: Instruction, lights: &mut [bool; WIDTH * HEIGHT]) {
    let op = instruction.operation;
    let dir = instruction.direction;

    let start_x = (dir.0).0;
    let end_x = (dir.1).0 + 1;

    let start_y = (dir.0).1;
    let end_y = (dir.1).1 + 1;

    for x in start_x..end_x {
        for y in start_y..end_y {
            let idx = x + (y * HEIGHT);

            match op {
                Operation::TurnOn => lights[idx] = true,
                Operation::TurnOff => lights[idx] = false,
                Operation::Toggle => lights[idx] = !lights[idx],
            }
        }
    }
}

fn calculate_part2(input: &str) -> u32 {
    let mut lights = [0; WIDTH * HEIGHT];

    for line in input.lines() {
        let instruction = Instruction::from(line);
        change_lights_part2(instruction, &mut lights);
    }

    lights.iter().fold(0, |acc, light| acc + (*light as u32))
}

fn change_lights_part2(instruction: Instruction, lights: &mut [u16; WIDTH * HEIGHT]) {
    let op = instruction.operation;
    let dir = instruction.direction;

    let start_x = (dir.0).0;
    let end_x = (dir.1).0 + 1;

    let start_y = (dir.0).1;
    let end_y = (dir.1).1 + 1;

    for x in start_x..end_x {
        for y in start_y..end_y {
            let idx = x + (y * HEIGHT);

            match op {
                Operation::TurnOn => lights[idx] += 1,
                Operation::Toggle => lights[idx] += 2,
                Operation::TurnOff => {
                    if lights[idx] > 0 {
                        lights[idx] -= 1;
                    }
                }
            }
        }
    }
}

type Point = (usize, usize);
type Direction = (Point, Point);

enum Operation {
    TurnOn,
    TurnOff,
    Toggle,
}

struct Instruction {
    operation: Operation,
    direction: Direction,
}

impl<'a> From<&'a str> for Instruction {
    fn from(input: &'a str) -> Self {
        let value = parse_operation(input);

        Instruction {
            operation: value.0,
            direction: value.1,
        }
    }
}

fn parse_operation(line: &str) -> (Operation, Direction) {
    let points: Vec<&str>;
    let operation;

    if line.starts_with("turn on") {
        points = line[8..].split(" through ").collect();
        operation = "turn on";
    } else if line.starts_with("turn off") {
        points = line[9..].split(" through ").collect();
        operation = "turn off";
    } else if line.starts_with("toggle") {
        points = line[7..].split(" through ").collect();
        operation = "toggle";
    } else {
        unreachable!();
    }

    debug_assert!(points.len() == 2, "Invalid input: {}", line);

    let point_from = parse_point(points[0]);
    let point_to = parse_point(points[1]);
    let direction = (point_from, point_to);

    match operation {
        "turn on" => (Operation::TurnOn, direction),
        "turn off" => (Operation::TurnOff, direction),
        "toggle" => (Operation::Toggle, direction),
        _ => unreachable!(),
    }
}

fn parse_point(pair: &str) -> Point {
    let values: Vec<_> = pair.split(",").collect();
    debug_assert!(values.len() == 2, "Invalid input: {}", pair);

    let coord_x: usize = values[0].parse().expect("Invalid input");
    let coord_y: usize = values[1].parse().expect("Invalid input");

    (coord_x, coord_y)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        assert_eq!(1000 * 1000, super::calculate_part1("turn on 0,0 through 999,999"));
    }

    #[test]
    fn part1_test2() {
        assert_eq!(1000, super::calculate_part1("toggle 0,0 through 999,0"));
    }

    #[test]
    fn part1_test3() {
        assert_eq!(0, super::calculate_part1("turn off 499,499 through 500,500"));
    }

    #[test]
    fn part2_test1() {
        assert_eq!(1, super::calculate_part2("turn on 0,0 through 0,0"));
    }

    #[test]
    fn part2_test2() {
        assert_eq!(2000000, super::calculate_part2("toggle 0,0 through 999,999"));
    }
}
