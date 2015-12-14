const INPUT: &'static str = include_str!("data/day8.txt");

pub fn part1() -> u32 {
    calculate_part1(INPUT)
}

pub fn part2() -> u32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u32 {
    let mut escape_next = false;
    let mut hex_digits = 0;
    let mut total = 0;

    for c in input.chars() {
        if c == '\\' && !escape_next {
            escape_next = true;
            continue
        } else if c == '"' && !escape_next {
            continue
        } else if c == 'x' && escape_next {
            hex_digits = 1;
            continue
        } else if hex_digits > 0 {
            if hex_digits == 1 {
                hex_digits = 2;
                continue
            } else if hex_digits == 2 {
                hex_digits = 0;
            }
        }

        escape_next = false;
        total += 1;
    }

    total
}

fn calculate_part2(input: &str) -> u32 {
    let mut escape_next = false;
    let mut total = 0;

    for c in input.chars() {
        if c == '\\' && !escape_next {
            escape_next = true;
            total += 1;
        } else if c == '\\' && escape_next {
            escape_next = false;
            total += 1; // escaped backslash becomes four backslashes
        } else if c == '"' && !escape_next {
            total += 2; // two for the added double quote at start and end of string
        } else if c == '"' && escape_next {
            total += 1;
            escape_next = false;
        } else if escape_next {
            escape_next = false;
        }

        total += 1;
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        assert_eq!(0, super::calculate_part1(r#""""#));
    }

    #[test]
    fn part1_test2() {
        assert_eq!(3, super::calculate_part1(r#""abc""#));
    }

    #[test]
    fn part1_test3() {
        assert_eq!(7, super::calculate_part1(r#""aaa\"aaa""#));
    }

    #[test]
    fn part1_test4() {
        assert_eq!(1, super::calculate_part1(r#""\x27""#));
    }

    #[test]
    fn part1_test5() {
        assert_eq!(1, super::calculate_part1(r#""\\""#));
    }

    #[test]
    fn part2_test1() {
        assert_eq!(6, super::calculate_part2(r#""""#));
    }

    #[test]
    fn part2_test2() {
        assert_eq!(9, super::calculate_part2(r#""abc""#));
    }

    #[test]
    fn part2_test3() {
        assert_eq!(16, super::calculate_part2(r#""aaa\"aaa""#));
    }

    #[test]
    fn part2_test4() {
        assert_eq!(11, super::calculate_part2(r#""\x27""#));
    }

    #[test]
    fn part2_test5() {
        assert_eq!(10, super::calculate_part2(r#""\\""#));
    }
}
