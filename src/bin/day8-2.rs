fn main() {
    let input = include_str!("day8.txt");
    // let input = r#""""#;
    // let input = r#""abc""#;
    // let input = r#""aaa\"aaa""#;
    // let input = r#""\x27""#;
    // let input = r#""\\""#;

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

    println!("{}", total - input.len());
}
