const INPUT: &'static str = include_str!("data/day1.txt");

pub fn part1() -> i32 {
    calculate_part1(INPUT)
}

pub fn part2() -> i32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> i32 {
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    };

    floor
}

fn calculate_part2(input: &str) -> i32 {
    let mut floor = 0;
    let mut position = 1;

    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }

        if floor < 0 {
            break;
        }

        position += 1;
    };

    position
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        assert_eq!(0, super::calculate_part1("(())"));
        assert_eq!(0, super::calculate_part1("()()"));
    }

    #[test]
    fn part1_test2() {
        assert_eq!(3, super::calculate_part1("((("));
        assert_eq!(3, super::calculate_part1("(()(()("));
    }

    #[test]
    fn part1_test3() {
        assert_eq!(3, super::calculate_part1("))((((("));
    }

    #[test]
    fn part1_test4() {
        assert_eq!(-1, super::calculate_part1("())"));
        assert_eq!(-1, super::calculate_part1("))("));
    }

    #[test]
    fn part1_test5() {
        assert_eq!(-3, super::calculate_part1(")))"));
        assert_eq!(-3, super::calculate_part1(")())())"));
    }

    #[test]
    fn part2_test1() {
        assert_eq!(1, super::calculate_part2(")"));
        assert_eq!(5, super::calculate_part2("()())"));
    }
}
