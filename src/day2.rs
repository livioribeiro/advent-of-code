const INPUT: &'static str = include_str!("data/day2.txt");

pub fn part1() -> u32 {
    calculate_part1(parse(INPUT))
}

pub fn part2() -> u32 {
    calculate_part2(parse(INPUT))
}

fn parse(input: &str) -> Vec<(u32, u32, u32)> {
    input.lines().map(|line| {
        let dimensions: Vec<_> = line.split('x').collect();
        debug_assert!(dimensions.len() == 3, "Invalid dimensions: {:?}", dimensions);

        let length: u32 = dimensions[0].parse().expect("Could not parse length");
        let width: u32 = dimensions[1].parse().expect("Could not parse width");
        let height: u32 = dimensions[2].parse().expect("Could not parse height");

        (length, width, height)
    })
    .collect()
}

fn calculate_part1(input: Vec<(u32, u32, u32)>) -> u32 {
    input.into_iter().fold(0, |acc, (length, width, height)| {
        let side_1 = length * width;
        let side_2 = width * height;
        let side_3 = length * height;

        let mut sides = [side_1, side_2, side_3];
        sides.sort();

        acc + (side_1 * 2) + (side_2 * 2) + (side_3 * 2) + sides[0]
    })
}

fn calculate_part2(input: Vec<(u32, u32, u32)>) -> u32 {
    input.into_iter().fold(0, |acc, (length, width, height)| {
        let mut dimensions = [length, width, height];
        dimensions.sort();

        let wrap = (dimensions[0] * 2) + (dimensions[1] * 2);
        let bow = dimensions.iter().fold(1, |acc, size| acc * size);

        acc + wrap + bow
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        assert_eq!(58, super::calculate_part1(vec![(2, 3, 4)]));
    }

    #[test]
    fn part1_test2() {
        assert_eq!(43, super::calculate_part1(vec![(1, 1, 10)]));
    }

    #[test]
    fn part2_test1() {
        assert_eq!(34, super::calculate_part2(vec![(2, 3, 4)]));
    }

    #[test]
    fn part2_test2() {
        assert_eq!(14, super::calculate_part2(vec![(1, 1, 10)]));
    }
}
