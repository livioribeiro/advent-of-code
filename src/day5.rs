const INPUT: &'static str = include_str!("data/day5.txt");

pub fn part1() -> u32 {
    calculate_part1(INPUT)
}

pub fn part2() -> u32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        if three_vowels(line) && two_in_a_row(line) && !forbidden(line) {
            total += 1;
        }
    }

    total
}

fn three_vowels(line: &str) -> bool {
    let mut total = 0;

    for c in line.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            total += 1;
        }
    }

    total >= 3
}

fn two_in_a_row(line: &str) -> bool {
    let chars1: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(1).collect();

    for (c1, c2) in chars1.iter().zip(chars2.iter()) {
        if c1 == c2 {
            return true
        }
    }

    false
}

fn forbidden(line: &str) -> bool {
    line.find("ab")
        .or(line.find("cd"))
        .or(line.find("pq"))
        .or(line.find("xy"))
        .is_some()
}

fn calculate_part2(input: &str) -> u32 {
    let mut total = 0;

    for line in input.lines() {
        if two_letters_twice(line) && two_with_one_between(line) {
            total += 1;
        }
    }

    total
}

fn two_letters_twice(line: &str) -> bool {
    let chars: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(1).collect();

    let size = chars.len();

    let mut i = 0;
    for (c1, c2) in chars.iter().zip(chars2.iter()) {
        // start inner loop two position ahead of outer loop
        let j = i + 2;

        // break if out of bounds
        if j > size - 1 {
            break;
        }

        for n in j..(size - 1) {
            if &chars[n] == c1 && &chars[n + 1] == c2 {
                return true
            }
        }

        i += 1;
    }

    false
}

fn two_with_one_between(line: &str) -> bool {
    let chars1: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(2).collect();

    for (c1, c2) in chars1.iter().zip(chars2.iter()) {
        if c1 == c2 {
            return true
        }
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        assert_eq!(1, super::calculate_part1("ugknbfddgicrmopn"));
        assert_eq!(1, super::calculate_part1("aaa"));
    }

    #[test]
    fn part1_test2() {
        assert_eq!(0, super::calculate_part1("jchzalrnumimnmhp"));
        assert_eq!(0, super::calculate_part1("haegwjzuvuyypxyu"));
        assert_eq!(0, super::calculate_part1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part2_test1() {
        assert_eq!(1, super::calculate_part2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn part2_test2() {
        assert_eq!(0, super::calculate_part2("uurcxstgmygtbstg"));
        assert_eq!(0, super::calculate_part2("ieodomkazucvgmuy"));
    }
}
