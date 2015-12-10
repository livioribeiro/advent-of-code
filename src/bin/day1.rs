fn main() {
    let input = include_str!("day1.txt");
    let mut floor = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        } else if c == ')' {
            floor -= 1;
        }
    };

    println!("{}", floor);
}
