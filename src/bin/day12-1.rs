const NUMERICAL: [char; 11] = ['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    let input = include_str!("day12.txt");

    let result = calculate(input);

    println!("{}", result);
}

fn calculate(input: &str) -> i32 {
    let mut reading_number = false;
    let mut buff = String::new();
    let mut total = 0;

    for c in input.chars() {
        if NUMERICAL.contains(&c) {
            reading_number = true;
            buff.push(c);
        } else if reading_number {
            reading_number = false;
            total += buff.parse().expect("Could no parse value");
            buff.clear();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_1() {
        assert_eq!(6, calculate("[1,2,3]"));
    }

    #[test]
    fn test_2() {
        assert_eq!(6, calculate(r#"{"a":2,"b":4}"#));
    }

    #[test]
    fn test_3() {
        assert_eq!(3, calculate(r#"[[[3]]]"#));
    }

    #[test]
    fn test_4() {
        assert_eq!(3, calculate(r#"{"a":{"b":4},"c":-1}"#));
    }

    #[test]
    fn test_5() {
        assert_eq!(0, calculate(r#"{"a":[-1,1]}"#));
    }

    #[test]
    fn test_6() {
        assert_eq!(0, calculate(r#"[-1,{"a":1}]"#));
    }

    #[test]
    fn test_7() {
        assert_eq!(0, calculate(r#"[]"#));
    }

    #[test]
    fn test_8() {
        assert_eq!(0, calculate(r#"{}"#));
    }
}
