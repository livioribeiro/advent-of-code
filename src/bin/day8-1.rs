fn main() {
    let input = include_str!("day8.txt");

    println!("{}", input.len() - calculate(input));
}

fn calculate(input: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::calculate;

    #[test]
    fn test_1() {
        assert_eq!(0, calculate(r#""""#));
    }

    #[test]
    fn test_2() {
        assert_eq!(3, calculate(r#""abc""#));
    }

    #[test]
    fn test_3() {
        assert_eq!(7, calculate(r#""aaa\"aaa""#));
    }

    #[test]
    fn test_4() {
        assert_eq!(1, calculate(r#""\x27""#));
    }

    #[test]
    fn test_5() {
        assert_eq!(1, calculate(r#""\\""#));
    }
}
