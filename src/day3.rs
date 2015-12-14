use std::collections::HashSet;

const INPUT: &'static str = include_str!("data/day3.txt");

pub fn part1() -> usize {
    calculate_part1(INPUT)
}

pub fn part2() -> usize {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> usize {
    let mut houses = HashSet::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    houses.insert((x, y));

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }

        houses.insert((x, y));
    }

    houses.len()
}

fn calculate_part2(input: &str) -> usize {
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

    houses.len()
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

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        assert_eq!(2, super::calculate_part1(">"));
    }

    #[test]
    fn part1_test2() {
        assert_eq!(4, super::calculate_part1("^>v<"));
    }

    #[test]
    fn part1_test3() {
        assert_eq!(2, super::calculate_part1("^v^v^v^v^v"));
    }

    #[test]
    fn part2_test1() {
        assert_eq!(3, super::calculate_part2("^v"));
    }

    #[test]
    fn part2_test2() {
        assert_eq!(3, super::calculate_part2("^>v<"));
    }

    #[test]
    fn part2_test3() {
        assert_eq!(11, super::calculate_part2("^v^v^v^v^v"));
    }
}
