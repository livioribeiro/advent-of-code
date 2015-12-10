use std::collections::HashSet;

fn main() {
    let input = include_str!("day3.txt");
    let mut houses = HashSet::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    houses.insert((x, y));

    for c in input.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => {}
        }

        houses.insert((x, y));
    }

    println!("{}", houses.len());
}
