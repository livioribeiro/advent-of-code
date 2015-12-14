extern crate regex;

use std::cmp;
use regex::Regex;

fn main() {
    let input = include_str!("day14.txt");

    let raindeers = parse(input);
    let winner_points = race(&raindeers, 2503);
    println!("{}", winner_points.iter().max().unwrap());
}

fn parse(input: &str) -> Vec<Raindeer> {
    let re = Regex::new(
        r#"(\w+?) can fly (\d+?) km/s for (\d+?) seconds, but then must rest for (\d+?) seconds."#
    ).expect("Invalid regex");

    let mut raindeers = vec![];

    for line in input.lines() {
        let cap = re.captures(line).expect("Invalid input");

        let name = cap.at(1).unwrap();
        let speed = cap.at(2).unwrap().parse().expect("Could not parse raindeer speed");
        let fly_time = cap.at(3).unwrap().parse().expect("Could not parse raindeer fly time");
        let rest_time = cap.at(4).unwrap().parse().expect("Could not parse raindeer rest time");

        raindeers.push(Raindeer::new(name, speed, fly_time, rest_time));
    }

    raindeers
}

fn race(raindeers: &Vec<Raindeer>, seconds: u32) -> Vec<u32> {
    let mut win_distance = 0;
    let mut distances: Vec<u32> = vec![0; raindeers.len()];
    let mut score: Vec<u32>  = vec![0; raindeers.len()];

    for s in 1..(seconds + 1) { // range last bound is exclusive
        // we don't know yet who has the lead
        for (i, raindeer) in raindeers.iter().enumerate() {
            let distance = raindeer.distance_at(s);
            distances[i] = distance;
            win_distance = cmp::max(win_distance, distance);
        }

        // now we know who has the lead
        for (i, d) in distances.iter().enumerate() {
            if *d == win_distance {
                score[i] += 1;
            }
        }
    }

    score
}

struct Raindeer {
    pub name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Raindeer {
    pub fn new(name: &str, speed: u32, fly_time: u32, rest_time: u32) -> Self {
        Raindeer {
            name: name.to_owned(),
            speed: speed,
            fly_time: fly_time,
            rest_time: rest_time,
        }
    }

    pub fn distance_at(&self, seconds: u32) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let rest = cmp::min(self.fly_time, (seconds % cycle_time));
        let effective_cycles = (seconds as f32 / cycle_time as f32).floor() as u32;

        (effective_cycles * self.fly_time * self.speed) + (rest * self.speed)
    }
}

#[cfg(test)]
mod tests {
    use super::Raindeer;

    #[test]
    fn comet_at_1000_seconds() {
        let comet = Raindeer::new("Comet", 14, 10, 127);
        assert_eq!(1120, comet.distance_at(1000));
    }

    #[test]
    fn dancer_at_1000_seconds() {
        let comet = Raindeer::new("Dancer", 16, 11, 162);
        assert_eq!(1056, comet.distance_at(1000));
    }

    #[test]
    fn comet_or_dancer() {
        let racers = vec![Raindeer::new("Comet", 14, 10, 127),
                          Raindeer::new("Dancer", 16, 11, 162)];

        let points = super::race(&racers, 1000);

        assert_eq!(312, points[0]);
        assert_eq!(689, points[1]);
    }
}
