use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("day9.txt");

    let mut cities: Vec<String> = Vec::new();
    let mut distancies: HashMap<String, u32> = HashMap::new();

    parse_cities(input, &mut cities, &mut distancies);

    let longest = longest_distance(&cities, &distancies);

    println!("{}", longest);
}

fn parse_cities(input: &str, cities: &mut Vec<String>, distancies: &mut HashMap<String, u32>) {
    for line in input.lines() {
        let parts: Vec<_> = line.split(" = ").collect();
        let city_vec: Vec<_> = parts[0].split(" to ").collect();
        let distance: u32 = parts[1].parse().expect("Could not parse distance");

        let city1 = city_vec[0].to_owned();
        let city2 = city_vec[1].to_owned();

        if !cities.contains(&city1) {
            cities.push(city1.to_owned());
        }

        if !cities.contains(&city2) {
            cities.push(city2.to_owned());
        }

        distancies.insert(format!("{}{}", city1, city2), distance);
        distancies.insert(format!("{}{}", city2, city1), distance);
    }
}

fn longest_distance(cities: &Vec<String>, distancies: &HashMap<String, u32>) -> u32 {
    if cities.len() == 1 {
        return 0
    }

    let mut distance = std::u32::MIN;

    for city in cities {
        let next_distance = longest_distance_from(&city, cities, distancies);
        distance = cmp::max(distance, next_distance)
    }

    distance
}

fn longest_distance_from(city: &str, cities: &Vec<String>, distancies: &HashMap<String, u32>) -> u32 {
    if cities.len() == 1 {
        return 0
    }

    let mut cities = cities.clone();
    let pos = cities.iter().position(|e| e == city).unwrap();
    let current = cities.remove(pos);

    let mut distance = std::u32::MIN;
    let mut next = String::new();

    for city in cities.iter() {
        let next_distance = distancies.get(&format!("{}{}", current, city)).unwrap().to_owned();
        if next_distance > distance {
            distance = next_distance;
            next = city.clone();
        }
    }

    distance + longest_distance_from(&next, &cities, distancies)
}
