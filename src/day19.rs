use std::cmp::Ordering;
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

fn calculate_part2(input: &str) -> usize {
    let (mut replacements, molecule) = parse(input);
    let mut steps = 0;

    let mut molecule = String::from(molecule);
    replacements.sort_by(|&(t1, r1), &(t2, r2)| {
        if !t1.contains("e") && t2.contains("e") {
            Ordering::Less
        } else if t1.contains("e") && !t2.contains("e") {
            Ordering::Greater
        } else {
            r1.cmp(r2).reverse()
        }
    });


    while molecule != "e" {
        for (target, replacement) in replacements.clone().into_iter() {
            if molecule.contains(replacement) {
                steps += molecule.split(replacement).count() - 1;
                molecule = molecule.replace(replacement, target);
                break
            }
        }
    }

    steps
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

        let (replacements, molecule) = super::parse(input);

        assert_eq!(3, replacements.len());
        assert_eq!("HOH", molecule);
    }

    #[test]
    fn part1_test1() {
        let molecule = "HOH";
        let replacements = vec![("H", "HO"), ("H", "OH"), ("O", "HH")];
        let mut variations: HashSet<String> = HashSet::new();

        for (target, replacement) in replacements.into_iter() {
            super::replace(molecule, target, replacement, &mut variations);
        }

        assert_eq!(4, variations.len());
    }

    #[test]
    fn part1_test2() {
        let molecule = "HOHOHO";
        let replacements = vec![("H", "HO"), ("H", "OH"), ("O", "HH")];
        let mut variations: HashSet<String> = HashSet::new();

        for (target, replacement) in replacements.into_iter() {
            super::replace(molecule, target, replacement, &mut variations);
        }

        assert_eq!(7, variations.len());
    }

    #[test]
    fn part2_test1() {
        let input = "e => H\n\
                     e => O\n\
                     H => HO\n\
                     H => OH\n\
                     O => HH\n\
                     \n\
                     HOH";

        let steps = super::calculate_part2(input);

        assert_eq!(3, steps);
    }

    #[test]
    fn part2_test2() {
        let input = "e => H\n\
                     e => O\n\
                     H => HO\n\
                     H => OH\n\
                     O => HH\n\
                     \n\
                     HOHOHO";

        let steps = super::calculate_part2(input);

        assert_eq!(6, steps);
    }
}
