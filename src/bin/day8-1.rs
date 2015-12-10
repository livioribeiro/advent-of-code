fn main() {
    let input = include_str!("day8.txt");

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

    println!("{}", input.len() - total);
}
