use regex::Regex;

const INPUT: &'static str = include_str!("data/day16.txt");

pub fn part1() -> u16 {
    calculate_part1(INPUT)
}

pub fn part2() -> u32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u16 {
    // let re = Regex::new("Sue (?P<sue>\\d+?):\
    //                     (,? children: (?P<children>\\d))\
    //                     |(,? cats: (?P<cats>\\d))\
    //                     |(,? samoyeds: (?P<samoyeds>\\d))\
    //                     |(,? pomeranians: (?P<pomeranians>\\d))\
    //                     |(,? akitas: (?P<akitas>\\d))\
    //                     |(,? vizslas: (?P<vizslas>\\d))\
    //                     |(,? goldfish: (?P<goldfish>\\d))\
    //                     |(,? trees: (?P<trees>\\d))\
    //                     |(,? cars: (?P<cars>\\d))\
    //                     |(,? perfumes: (?P<perfumes>\\d))"
    // ).expect("Invalid regex");

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
        let current_sue = cap.at(1).expect("Invalid input 2");

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

fn calculate_part2(_: &str) -> u32 {
    0
}
