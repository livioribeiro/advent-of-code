use std::env;

fn main() {
    let input = "1321131112";

    let mut answer = String::from(input);

    let iterations: u32 = env::args().nth(1).unwrap_or("40".to_owned()).parse().expect("Wrong argument");
    for _ in 0..iterations {
        answer = look_and_say(&answer);
    }

    println!("{}", answer.len());
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
