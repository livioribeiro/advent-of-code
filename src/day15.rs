use std::cmp;
use regex::Regex;

const INPUT: &'static str = include_str!("data/day15.txt");

pub fn part1() -> u32 {
    calculate_part1(INPUT)
}

pub fn part2() -> u32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u32 {
    let ingredients = parse(input);
    let mut total = 0;

    let combs = Box::new(combinations(ingredients.len() as u16, 100));

    for comb in combs.iter() {
        let mut cap = 0;
        let mut dur = 0;
        let mut fla = 0;
        let mut tex = 0;

        for (ingr, amount) in ingredients.iter().zip(comb.iter()) {
            cap += (ingr.capacity as i32) * (*amount as i32);
            dur += (ingr.durability as i32) * (*amount as i32);
            fla += (ingr.flavor as i32) * (*amount as i32);
            tex += (ingr.texture as i32) * (*amount as i32);
        }

        cap = cmp::max(0, cap);
        dur = cmp::max(0, dur);
        fla = cmp::max(0, fla);
        tex = cmp::max(0, tex);

        let score = cap * dur * fla * tex;
        total = cmp::max(total, score);
    }

    total as u32
}

fn calculate_part2(input: &str) -> u32 {
    let ingredients = parse(input);

    let mut total = 0;

    let combs = Box::new(combinations(ingredients.len() as u16, 100));

    for comb in combs.iter() {
        let mut cap = 0;
        let mut dur = 0;
        let mut fla = 0;
        let mut tex = 0;
        let mut cal = 0;

        for (ingr, amount) in ingredients.iter().zip(comb.iter()) {
            cap += (ingr.capacity as i32) * (*amount as i32);
            dur += (ingr.durability as i32) * (*amount as i32);
            fla += (ingr.flavor as i32) * (*amount as i32);
            tex += (ingr.texture as i32) * (*amount as i32);
            cal += (ingr.calories as i32) * (*amount as i32);
        }

        cap = cmp::max(0, cap);
        dur = cmp::max(0, dur);
        fla = cmp::max(0, fla);
        tex = cmp::max(0, tex);

        if cal == 500 {
            let score = cap * dur * fla * tex;
            total = cmp::max(total, score);
        }

    }

    total as u32
}

fn parse(input: &str) -> Vec<Ingredient> {
    let re = Regex::new(
        r#"(\w+?): capacity (-?\d+?), durability (-?\d+?), flavor (-?\d+?), texture (-?\d+?), calories (-?\d+?)"#
    ).expect("Invalid regex");

    input.lines().map(|line| {
        let cap = re.captures(line).expect("Regex didn't match");

        let name = cap.at(1).expect("No capture for 'name'");

        let capacity: i16 = cap.at(2).expect("No capture for 'capacity'")
            .parse().expect("Could no parse capacity");

        let durability: i16 = cap.at(3).expect("No capture for 'durability'")
            .parse().expect("Could no parse durability");

        let flavor: i16 = cap.at(4).expect("No capture for 'flavor'")
            .parse().expect("Could no parse flavor");

        let texture: i16 = cap.at(5).expect("No capture for 'texture'")
            .parse().expect("Could no parse texture");

        let calories: i16 = cap.at(6).expect("No capture for 'calories'")
            .parse().expect("Could no parse calories");

        Ingredient::new(name, capacity, durability, flavor, texture, calories)
    }).collect()
}

fn combinations(slots: u16, teaspoons: u16) -> Vec<Vec<u16>> {
    let mut result = vec![];
    combinations_recursive(slots, teaspoons + 1, &mut vec![], &mut result);

    result
}

fn combinations_recursive(slots: u16, remaining: u16, acc: &mut Vec<u16>, result: &mut Vec<Vec<u16>>) {
    if acc.len() as u16 == slots {
        if acc.iter().fold(0, |acc, elem| acc + elem) == 100 {
            result.push(acc.clone());
        }
        return
    }

    for i in 0..remaining {
        acc.push(i);
        combinations_recursive(slots, remaining - i, acc, result);
        acc.pop();
    }
}

#[derive(Clone, Debug)]
struct Ingredient {
    pub name: String,
    pub capacity: i16,
    pub durability: i16,
    pub flavor: i16,
    pub texture: i16,
    pub calories: i16,
}

impl Ingredient {
    pub fn new(name: &str,
               capacity: i16,
               durability: i16,
               flavor: i16,
               texture: i16,
               calories: i16)
               -> Self
    {
        Ingredient {
            name: name.to_owned(),
            capacity: capacity,
            durability: durability,
            flavor: flavor,
            texture: texture,
            calories: calories,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                     Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let result = super::calculate_part1(input);
        assert_eq!(62842880, result);
    }

    #[test]
    fn test_part2() {
        let input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                     Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let result = super::calculate_part2(input);
        assert_eq!(57600000, result);
    }
}
