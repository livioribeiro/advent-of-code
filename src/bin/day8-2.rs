fn main() {
    let input = include_str!("day8.txt");

    println!("{}", calculate(input) - input.len());
}

fn calculate(input: &str) -> usize {
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
    use super::calculate;

    #[test]
    fn test_1() {
        assert_eq!(6, calculate(r#""""#));
    }

    #[test]
    fn test_2() {
        assert_eq!(9, calculate(r#""abc""#));
    }

    #[test]
    fn test_3() {
        assert_eq!(16, calculate(r#""aaa\"aaa""#));
    }

    #[test]
    fn test_4() {
        assert_eq!(11, calculate(r#""\x27""#));
    }

    #[test]
    fn test_5() {
        assert_eq!(10, calculate(r#""\\""#));
    }
}
