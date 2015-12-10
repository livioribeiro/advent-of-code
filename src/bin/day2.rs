fn main() {
    let input = include_str!("day2.txt");

    let mut total = 0;

    for line in input.lines() {
        let dimensions: Vec<_> = line.split('x').collect();
        debug_assert!(dimensions.len() == 3, "Invalid dimensions: {:?}", dimensions);

        let length: u32 = dimensions[0].parse().expect("Could not parse length");
        let width: u32 = dimensions[1].parse().expect("Could not parse width");
        let height: u32 = dimensions[2].parse().expect("Could not parse height");

        total += calculate(length, width, height);
    }

    println!("{}", total);
}

fn calculate(length: u32, width: u32, height: u32) -> u32 {
    let side_1 = length * width;
    let side_2 = width * height;
    let side_3 = length * height;

    let mut smallest = side_1;

    for side in vec![side_2, side_3] {
        if side < smallest {
            smallest = side;
        }
    }

    (side_1 * 2) + (side_2 * 2) + (side_3 * 2) + smallest
}
