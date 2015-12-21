use std::collections::HashSet;

const INPUT: &'static str = include_str!("data/day19.txt");

pub fn part1() -> usize {
    calculate_part1(INPUT)
}

pub fn part2() -> usize {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> usize {
    let (replacements, molecule) = parse(input);
    let mut variations: HashSet<String> = HashSet::new();

    for (target, replacement) in replacements.into_iter() {
        replace(molecule, target, replacement, &mut variations);
    }

    variations.len()
}

fn calculate_part2(_: &str) -> usize {
    0
}

fn parse(input: &str) -> (Vec<(&str, &str)>, &str) {
    let mut replacements: Vec<(&str, &str)> = Vec::new();
    let mut molecule = "";
    let mut last_line = false;

    for line in input.lines() {
        if last_line {
            molecule = line;
            break
        }

        if line.is_empty() {
            last_line = true;
            continue
        }

        let replacement: Vec<&str> = line.split(" => ").collect();
        replacements.push((replacement[0], replacement[1]));
    }

    (replacements, molecule)
}

fn replace(input: &str, target: &str, replacement: &str, variations: &mut HashSet<String>) {
    let splits: Vec<_> = input.split(target).collect();

    if splits.len() < 2 {
        return
    }

    for i in 1..splits.len() {
        let mut result = String::from(splits[0]);
        for j in 1..splits.len() {
            if i == j {
                result.push_str(replacement);
            } else {
                result.push_str(target);
            }
            result.push_str(splits[j]);
        }
        variations.insert(result);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    #[test]
    fn parse() {
        let input = "H => HO\n\
                     H => OH\n\
                     O => HH\n\
                     \n\
                     HOH";

        let (replacements, _) = super::parse(input);
        println!("{:?}", replacements);
        assert_eq!(3, replacements.len());
    }

    #[test]
    fn part1_test1() {
        let input = "H => HO\n\
                     H => OH\n\
                     O => HH\n\
                     \n\
                     HOH";

        let (replacements, molecule) = super::parse(input);
        let mut variations: HashSet<String> = HashSet::new();

        for (target, replacement) in replacements.into_iter() {
            super::replace(molecule, target, replacement, &mut variations);
        }

        assert_eq!(4, variations.len());
    }
}
