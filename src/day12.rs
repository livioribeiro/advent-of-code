use std::str::Chars;

const INPUT: &'static str = include_str!("data/day12.txt");
const NUMERICAL: [char; 11] = ['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn part1() -> i32 {
    calculate_part1(INPUT)
}

pub fn part2() -> i32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> i32 {
    let mut reading_number = false;
    let mut buff = String::new();
    let mut total = 0;

    for c in input.chars() {
        if NUMERICAL.contains(&c) {
            reading_number = true;
            buff.push(c);
        } else if reading_number {
            reading_number = false;
            total += buff.parse().expect("Could not parse value");
            buff.clear();
        }
    }

    total
}

fn calculate_part2(input: &str) -> i32 {
    calculate_recursive(&mut input.chars())
}

fn calculate_recursive(input: &mut Chars) -> i32 {
    let mut reading_number = false;
    let mut buff = String::new();
    let mut total = 0;

    let mut red = Red::new();

    while let Some(c) = input.next() {
        red.input(c);

        if !red.check() {
            let mut stack = 1;
            // read until close current structure
            while let Some(c) = input.next() {
                if stack == 0 {
                    break
                } else if c == '{' {
                    stack += 1;
                } else if c == '}' {
                    stack -= 1;
                }
            }

            total = 0;
            break
        }

        if NUMERICAL.contains(&c) {
            reading_number = true;
            buff.push(c);
        } else if reading_number {
            reading_number = false;
            total += buff.parse().expect("Could not parse value");
            buff.clear();
        }

        if c == '{' {
            total += calculate_recursive(input);
        } else if c == '}' {
            return total
        }
    }

    total
}

struct Red {
    pub level: u32,
}

impl Red {
    fn new() -> Self {
        Red {
            level: 0,
        }
    }

    pub fn input(&mut self, c: char) {
        if self.level == 6 {
            return
        }

        if c == ':' && self.level == 0
            || c == '"' && self.level == 1
            || c == 'r' && self.level == 2
            || c == 'e' && self.level == 3
            || c == 'd' && self.level == 4
            || c == '"' && self.level == 5
        {
            self.level += 1;
        } else if self.level < 6 {
            self.level = 0;
        }
    }

    pub fn check(&self) -> bool {
        self.level < 5
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        assert_eq!(6, super::calculate_part1("[1,2,3]"));
    }

    #[test]
    fn part1_test2() {
        assert_eq!(6, super::calculate_part1(r#"{"a":2,"b":4}"#));
    }

    #[test]
    fn part1_test3() {
        assert_eq!(3, super::calculate_part1(r#"[[[3]]]"#));
    }

    #[test]
    fn part1_test4() {
        assert_eq!(3, super::calculate_part1(r#"{"a":{"b":4},"c":-1}"#));
    }

    #[test]
    fn part1_test5() {
        assert_eq!(0, super::calculate_part1(r#"{"a":[-1,1]}"#));
    }

    #[test]
    fn part1_test6() {
        assert_eq!(0, super::calculate_part1(r#"[-1,{"a":1}]"#));
    }

    #[test]
    fn part1_test7() {
        assert_eq!(0, super::calculate_part1(r#"[]"#));
    }

    #[test]
    fn part1_test8() {
        assert_eq!(0, super::calculate_part1(r#"{}"#));
    }

    #[test]
    fn part2_test1() {
        assert_eq!(6, super::calculate_part2("[1,2,3]"));
    }

    #[test]
    fn part2_test2() {
        assert_eq!(6, super::calculate_part2(r#"{"a":2,"b":4}"#));
    }

    #[test]
    fn part2_test3() {
        assert_eq!(3, super::calculate_part2(r#"[[[3]]]"#));
    }

    #[test]
    fn part2_test4() {
        assert_eq!(3, super::calculate_part2(r#"{"a":{"b":4},"c":-1}"#));
    }

    #[test]
    fn part2_test5() {
        assert_eq!(0, super::calculate_part2(r#"{"a":[-1,1]}"#));
    }

    #[test]
    fn part2_test6() {
        assert_eq!(0, super::calculate_part2(r#"[-1,{"a":1}]"#));
    }

    #[test]
    fn part2_test7() {
        assert_eq!(0, super::calculate_part2(r#"[]"#));
    }

    #[test]
    fn part2_test8() {
        assert_eq!(0, super::calculate_part2(r#"{}"#));
    }

    #[test]
    fn part2_test_red1() {
        assert_eq!(4, super::calculate_part2(r#"[1,{"c":"red","b":2},3]"#));
    }

    #[test]
    fn part2_test_red2() {
        assert_eq!(0, super::calculate_part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#));
    }

    #[test]
    fn part2_test_red3() {
        assert_eq!(6, super::calculate_part2(r#"[1,"red",5]"#));
    }
}
