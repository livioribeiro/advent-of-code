use regex::Regex;

const INPUT: &'static str = include_str!("data/day16.txt");

pub fn part1() -> u16 {
    calculate_part1(INPUT)
}

pub fn part2() -> u16 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u16 {
    let re_sue = Regex::new("Sue (\\d+):").expect("Invalid regex (Sue)");

    let tests = [
        Regex::new("children: 3").expect("Invalid regex(children)"),
        Regex::new("cats: 7").expect("Invalid regex(cats)"),
        Regex::new("samoyeds: 2").expect("Invalid regex(samoyeds)"),
        Regex::new("pomeranians: 3").expect("Invalid regex(pomeranians)"),
        Regex::new("akitas: 0").expect("Invalid regex(akitas)"),
        Regex::new("vizslas: 0").expect("Invalid regex(vizslas)"),
        Regex::new("goldfish: 5").expect("Invalid regex(goldfish)"),
        Regex::new("trees: 3").expect("Invalid regex(trees)"),
        Regex::new("cars: 2").expect("Invalid regex(cars)"),
        Regex::new("perfumes: 1").expect("Invalid regex(perfumes)"),
    ];

    let mut max_points = 0;
    let mut sue: u16 = 0;

    for line in input.lines() {
        let mut current_points = 0;
        let cap = re_sue.captures(line).expect(&format!("Invalid input: {}", line));
        let current_sue = cap.at(1).unwrap();

        for test in tests.iter() {
            if test.is_match(line) {
                current_points += 1;
            }
        }

        if current_points > max_points {
            max_points = current_points;
            sue = current_sue.parse().expect(&format!("Could not parse aunt Sue number: {}", current_sue));
        }
    }

    sue
}

fn calculate_part2(input: &str) -> u16 {
    let re_sue = Regex::new("Sue (\\d+):").expect("Invalid regex (Sue)");

    let tests = [
        Regex::new("children: 3").expect("Invalid regex(children)"),
        Regex::new("samoyeds: 2").expect("Invalid regex(samoyeds)"),
        Regex::new("akitas: 0").expect("Invalid regex(akitas)"),
        Regex::new("vizslas: 0").expect("Invalid regex(vizslas)"),
        Regex::new("cars: 2").expect("Invalid regex(cars)"),
        Regex::new("perfumes: 1").expect("Invalid regex(perfumes)"),
    ];

    let re_cats = Regex::new("cats: (\\d+?)").expect("Invalid regex(cats)");
    let re_trees = Regex::new("trees: (\\d+?)").expect("Invalid regex(trees)");
    let re_pomeranians = Regex::new("pomeranians: (\\d+?)").expect("Invalid regex(pomeranians)");
    let re_goldfish = Regex::new("goldfish: (\\d+?)").expect("Invalid regex(goldfish)");

    let mut max_points = 0;
    let mut sue: u16 = 0;

    for line in input.lines() {
        let mut current_points = 0;
        let cap = re_sue.captures(line).expect(&format!("Invalid input: {}", line));
        let current_sue = cap.at(1).unwrap();

        for test in tests.iter() {
            if test.is_match(line) {
                current_points += 1;
            }
        }

        if let Some(cap) = re_cats.captures(line) {
            let val: u16 = cap.at(1).unwrap().parse().expect("Could not parse value");
            if val > 7 {
                current_points += 1;
            }
        }

        if let Some(cap) = re_trees.captures(line) {
            let val: u16 = cap.at(1).unwrap().parse().expect("Could not parse value");
            if val > 3 {
                current_points += 1;
            }
        }

        if let Some(cap) = re_pomeranians.captures(line) {
            let val: u16 = cap.at(1).unwrap().parse().expect("Could not parse value");
            if val < 3 {
                current_points += 1;
            }
        }

        if let Some(cap) = re_goldfish.captures(line) {
            let val: u16 = cap.at(1).unwrap().parse().expect("Could not parse value");
            if val < 5 {
                current_points += 1;
            }
        }

        if current_points > max_points {
            max_points = current_points;
            sue = current_sue.parse().expect(&format!("Could not parse aunt Sue number: {}", current_sue));
        }
    }

    sue
}
