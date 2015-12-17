const INPUT: &'static str = include_str!("data/day17.txt");

pub fn part1() -> u16 {
    calculate_part1(INPUT)
}

pub fn part2() -> u32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u16 {
    let jugs = parse(input);
    calculate_part1_recursive(&jugs, 0, 150)
}

fn calculate_part1_recursive(jugs: &[u16], acc: u16, expected: u16) -> u16 {
    if jugs.len() == 0 {
        return 0
    }

    let mut total = 0;

    if jugs[0] + acc == expected {
        total + 1;
    }

    for (i, j) in jugs.iter().enumerate() {
        if j + acc == expected {
            total += 1
        }

        total += calculate_part1_recursive(&jugs[(i + 1)..], acc + j, expected);
    }

    total
}

fn calculate_part2(_: &str) -> u32 {
    0
}

fn parse(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse::<u16>().expect("Invalid input")).collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        let input = vec![20, 15, 10, 5, 5];
        assert_eq!(4, super::calculate_part1_recursive(&input, 0, 25));
    }
}
