const INPUT: &'static str = include_str!("data/day17.txt");

pub fn part1() -> u16 {
    calculate_part1(INPUT)
}

pub fn part2() -> u16 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u16 {
    let jugs = parse(input);
    num_combinations(&jugs, 150)
}

fn num_combinations(jugs: &[u16], expected: u16) -> u16 {
    num_combinations_recursive(&jugs, 0, expected)
}

fn num_combinations_recursive(jugs: &[u16], acc: u16, expected: u16) -> u16 {
    if jugs.len() == 0 {
        return 0
    }

    let mut total = 0;

    for (i, j) in jugs.iter().enumerate() {
        if j + acc == expected {
            total += 1;
        }

        total += num_combinations_recursive(&jugs[(i + 1)..], acc + j, expected);
    }

    total
}

fn calculate_part2(input: &str) -> u16 {
    let jugs = parse(input);
    num_combinations_min(&jugs, 150)
}

fn num_combinations_min(jugs: &[u16], expected: u16) -> u16 {
    let mut total = 0;

    for i in 1..(jugs.len() + 1) {
        total += combinations(&jugs, i as u16)
            .into_iter()
            .filter(|c| c.iter().fold(0, |acc, i| acc + i) == expected)
            .count();

        if total > 0 {
            break
        }
    }

    total as u16
}

fn parse(input: &str) -> Vec<u16> {
    input.lines().map(|l| l.parse::<u16>().expect("Invalid input")).collect()
}

fn combinations(input: &[u16], slots: u16) -> Vec<Vec<u16>> {
    let mut result = vec![];
    let mut input = Vec::from(input);
    input.sort();
    input.reverse();
    combinations_recursive(&input, slots, &mut vec![], &mut result);

    result
}

fn combinations_recursive(input: &[u16], slots: u16, acc: &mut Vec<u16>, result: &mut Vec<Vec<u16>>) {
    if acc.len() as u16 == slots {
        result.push(acc.clone());
        return
    }

    for (i, elem) in input.iter().enumerate() {
        acc.push(*elem);
        combinations_recursive(&input[(i+1)..], slots, acc, result);
        acc.pop();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        let input = vec![20, 15, 10, 5, 5];
        assert_eq!(4, super::num_combinations(&input, 25));
    }

    #[test]
    fn part2_test2() {
        let input = vec![20, 15, 10, 5, 5];
        assert_eq!(3, super::num_combinations_min(&input, 25,));
    }
}
