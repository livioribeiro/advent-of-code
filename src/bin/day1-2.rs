fn main() {
    let input = include_str!("day1.txt");

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

    println!("{}", position);
}
