extern crate regex;

use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("day13.txt");

    let result = calculate(input);

    println!("{}", result);
}

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
    let (mut group, mut happiness) = parse(input);

    for person in group.iter() {
        happiness.insert(format!("{}MySelf", person), 0);
        happiness.insert(format!("MySelf{}", person), 0);
    }

    group.push("MySelf".to_owned());

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
