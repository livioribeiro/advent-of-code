extern crate regex;

use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("day13.txt");

    let result = calculate(input);

    println!("{}", result);
}

// fn calculate(input: &str) -> i32 {
//     let (group, happiness) = parse(input);
//
//     let mut total = std::i32::MIN;
//     for i in 0..group.len() {
//         let prev = {
//             if i == 0 {
//                 group.last().unwrap()
//             } else {
//                 group.get(i - 1).unwrap()
//             }
//         };
//
//         let current = group.get(i).unwrap();
//
//         let result = calculate_recursive(current, prev, current, &group, &happiness);
//         println!("{}", result);
//         total = cmp::max(total, result);
//     }
//
//     total
// }

// fn calculate_recursive(first: &str,
//                        prev: &str,
//                        current: &str,
//                        group: &Vec<String>,
//                        happiness: &HashMap<String, i32>) -> i32
// {
//     let pos = group.iter().position(|p| p == current).unwrap();
//
//     let mut group = group.clone();
//     group.remove(pos);
//
//     if group.len() == 1 {
//         let ref next = group[0];
//         let next_id = format!("{}{}", current, next);
//         let prev_id = format!("{}{}", current, prev);
//
//         let gain = happiness[&prev_id] + happiness[&next_id];
//
//         let next_id = format!("{}{}", next, first);
//         let prev_id = format!("{}{}", next, current);
//
//         return gain + happiness[&prev_id] + happiness[&next_id];
//     }
//
//     let mut gain = std::i32::MIN;
//     let mut next = String::new();
//
//     for person in group.iter() {
//         let next_id = format!("{}{}", current, person);
//         let prev_id = format!("{}{}", current, prev);
//
//         let current_gain = happiness[&prev_id] + happiness[&next_id];
//
//         if current_gain > gain {
//             gain = current_gain;
//             next = person.to_owned();
//         }
//     }
//
//     gain + calculate_recursive(first, current, &next, &group, happiness)
// }

fn parse(input: &str) -> (Vec<String>, HashMap<String, i32>) {
    let mut group: Vec<String> = Vec::new();
    let mut happiness: HashMap<String, i32> = HashMap::new();

    let re = regex::Regex::new(
        r#"(\w+?) would (gain|lose) (\d{1,2}) happiness units by sitting next to (\w+?)\."#
    ).expect("Invalid regex");

    for line in input.lines() {
        let cap = re.captures(line).expect(&format!("Invalid input: {}", line));

        let factor = match cap.at(2).unwrap() {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        };

        let person1 = cap.at(1).unwrap().to_owned();
        let person2 = cap.at(4).unwrap().to_owned();

        let gain: i32 = cap.at(3).unwrap().parse().expect("Invalid input");
        happiness.insert(format!("{}{}", &person1, &person2), gain * factor);

        if !group.contains(&person1) {
            group.push(person1);
        }

        if !group.contains(&person2) {
            group.push(person2);
        }

    }

    (group, happiness)
}

fn calculate_happiness(group: &Vec<String>, happiness: &HashMap<String, i32>) -> i32 {
    let mut total = 0;

    for i in 0..group.len() {
        let prev = {
            if i == 0 {
                group.len() - 1
            } else {
                i - 1
            }
        };

        let next = {
            if i == group.len() - 1 {
                0
            } else {
                i + 1
            }
        };

        let prev = format!("{}{}", group[i], group[prev]);
        let next = format!("{}{}", group[i], group[next]);

        total += happiness[&prev] + happiness[&next];
    }

    total
}

fn calculate(input: &str) -> i32 {
    let (group, happiness) = parse(input);

    let mut result = std::i32::MIN;
    for group in permutations(&group) {
        result = cmp::max(result, calculate_happiness(&group, &happiness));
    }

    result
}

fn permutations<T>(input: &Vec<T>) -> Vec<Vec<T>>
    where T: Clone + PartialEq
{
    let mut result = vec![];
    permutations_recursive(input, &vec![], &mut vec![], &mut result);

    result
}

fn permutations_recursive<T>(original: &Vec<T>, input: &Vec<T>, acc: &mut Vec<T>, result: &mut Vec<Vec<T>>)
    where T: Clone + PartialEq
{
    if acc.len() == original.len() {
        result.push(acc.clone());
        return
    }

    for i in original {
        if !input.contains(i) {
            acc.push(i.clone());
            let mut input = input.clone();
            input.push(i.clone());
            permutations_recursive(original, &input, acc, result);
            acc.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_line() {
        let input = "Alice would lose 2 happiness units by sitting next to Bob.";
        let (people, happiness) = super::parse(input);

        let alice = "Alice".to_owned();
        let bob = "Bob".to_owned();

        assert_eq!(2, people.len());
        assert!(people.contains(&alice));
        assert!(people.contains(&bob));
        assert_eq!(-2, happiness["AliceBob"]);
    }

    #[test]
    fn parse_two_lines() {
        let input = "Alice would lose 2 happiness units by sitting next to Bob.\n\
                     Bob would gain 93 happiness units by sitting next to Alice.";

        let (people, happiness) = super::parse(input);

        let alice = "Alice".to_owned();
        let bob = "Bob".to_owned();

        assert_eq!(2, people.len());
        assert!(people.contains(&alice));
        assert!(people.contains(&bob));
        assert_eq!(-2, happiness["AliceBob"]);
        assert_eq!(93, happiness["BobAlice"]);
    }

    #[test]
    fn calculate() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.\n\
                     Alice would lose 79 happiness units by sitting next to Carol.\n\
                     Alice would lose 2 happiness units by sitting next to David.\n\
                     Bob would gain 83 happiness units by sitting next to Alice.\n\
                     Bob would lose 7 happiness units by sitting next to Carol.\n\
                     Bob would lose 63 happiness units by sitting next to David.\n\
                     Carol would lose 62 happiness units by sitting next to Alice.\n\
                     Carol would gain 60 happiness units by sitting next to Bob.\n\
                     Carol would gain 55 happiness units by sitting next to David.\n\
                     David would gain 46 happiness units by sitting next to Alice.\n\
                     David would lose 7 happiness units by sitting next to Bob.\n\
                     David would gain 41 happiness units by sitting next to Carol.";

        let result = super::calculate(input);
        assert_eq!(330, result);
    }
}
