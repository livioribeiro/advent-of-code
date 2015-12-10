use std::convert::From;

const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

fn main() {
    let input = include_str!("day6.txt");

    let mut lights = [false; WIDTH * HEIGHT];

    for line in input.lines() {
        let instruction = Instruction::from(line);
        change_lights(instruction, &mut lights);
    }

    let total = lights.iter().fold(0, |acc, light| if *light { acc + 1 } else { acc });
    println!("{}", total);
}

fn change_lights(instruction: Instruction, lights: &mut [bool; WIDTH * HEIGHT]) {
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
