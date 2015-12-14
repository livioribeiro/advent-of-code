const INPUT: &'static str = "1321131112";
const ITERATIONS_PART1: u32 = 40;
const ITERATIONS_PART2: u32 = 50;

pub fn part1() -> usize {
    calculate_part1(INPUT)
}

pub fn part2() -> usize {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> usize {
    let mut answer = String::from(input);

    for _ in 0..ITERATIONS_PART1 {
        answer = look_and_say(&answer);
    }

    answer.len()
}

fn calculate_part2(input: &str) -> usize {
    let mut answer = String::from(input);

    for _ in 0..ITERATIONS_PART2 {
        answer = look_and_say(&answer);
    }

    answer.len()
}

fn look_and_say(input: &str) -> String {
    let mut iter = input.chars();

    let mut output = String::new();
    let mut current = match iter.next() {
        Some(c) => c,
        None => return output,
    };

    let mut times = 1;

    for c in iter {
        if c == current {
            times += 1;
        } else {
            output.push_str(&format!("{}{}", times, current));
            current = c;
            times = 1;
        }
    }

    // push the last element
    output.push_str(&format!("{}{}", times, current));

    output
}

#[cfg(test)]
mod tests {
    use super::look_and_say;

    #[test]
    fn test_1() {
        assert_eq!("11", look_and_say("1"));
    }

    #[test]
    fn test_11() {
        assert_eq!("21", look_and_say("11"));
    }

    #[test]
    fn test_21() {
        assert_eq!("1211", look_and_say("21"));
    }

    #[test]
    fn test_1211() {
        assert_eq!("111221", look_and_say("1211"));
    }

    #[test]
    fn test_111221() {
        assert_eq!("312211", look_and_say("111221"));
    }
}
