use std::collections::HashSet;

fn main() {
    let input = include_str!("day3.txt");

    let mut houses = HashSet::new();

    let mut santa = (0, 0);
    let mut robo = (0, 0);

    houses.insert(santa);
    houses.insert(robo);

    let mut turn = Turn::Santa;

    for c in input.chars() {
        match turn {
            Turn::Santa => {
                move_santa(&mut santa, c);
                houses.insert(santa);
                turn = Turn::Robo;
            }
            Turn::Robo => {
                move_santa(&mut robo, c);
                houses.insert(robo);
                turn = Turn::Santa;
            }
        }
    }

    println!("{}", houses.len());
}

enum Turn {
    Santa,
    Robo,
}

fn move_santa(subject: &mut (i32, i32), direction: char) {
    match direction {
        '>' => subject.0 += 1,
        '<' => subject.0 -= 1,
        '^' => subject.1 += 1,
        'v' => subject.1 -= 1,
        _ => {}
    }
}
