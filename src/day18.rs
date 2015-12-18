const INPUT: &'static str = include_str!("data/day18.txt");
const STEPS: u32 = 100;

pub fn part1() -> u32 {
    calculate_part1(INPUT)
}

pub fn part2() -> u32 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u32 {
    let mut lights = parse(input);

    for _ in 0..STEPS {
        lights.animate();
    }

    lights.lights_on() as u32
}

fn calculate_part2(input: &str) -> u32 {
    let mut lights = parse(input);

    for _ in 0..STEPS {
        lights.animate2();
    }

    lights.lights_on() as u32
}

fn parse(input: &str) -> LightMatrix {
    let mut lights = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for line in input.lines() {
        width = line.len();
        for c in line.chars() {
            if c == '#' {
                lights.push(true);
            } else {
                lights.push(false);
            }
        }

        height += 1;
    }

    LightMatrix::new(lights, width, height)
}

#[derive(Clone, Debug)]
struct LightMatrix {
    width: usize,
    height: usize,
    lights: Vec<bool>,
}

#[allow(dead_code)]
impl LightMatrix {
    pub fn new(lights: Vec<bool>, width: usize, height: usize) -> Self {
        LightMatrix {
            width: width,
            height: height,
            lights: lights,
        }
    }

    pub fn toggle(&mut self, x: usize, y: usize) -> bool {
        let idx = y * self.height + x;
        self.lights[idx] = !self.lights[idx];
        self.lights[idx]
    }

    pub fn turn(&mut self, x: usize, y: usize, state: bool) {
        let idx = y * self.height + x;
        self.lights[idx] = state;
    }

    pub fn turn_on(&mut self, x: usize, y: usize) {
        self.turn(x, y, true);
    }

    pub fn turn_off(&mut self, x: usize, y: usize) {
        self.turn(x, y, false);
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let idx = y * self.height + x;
        self.lights[idx]
    }

    pub fn lights_on(&self) -> usize {
        self.lights.iter().fold(0, |acc, e| if *e {acc + 1} else {acc})
    }

    pub fn lights_on_around(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;
        let width = self.width as isize;
        let height = self.height as isize;

        let lights_around = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

        lights_around.iter().fold(0, |acc, &(lx, ly)| {
            if (x + lx < 0) || (y + ly < 0) || (x + lx == width) || (y + ly == height) {
                acc
            } else if self.get((x + lx) as usize, (y + ly) as usize) {
                acc + 1
            } else {
                acc
            }
        })
    }

    pub fn animate(&mut self) {
        let lights = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let on_around = lights.lights_on_around(x, y);

                if lights.get(x, y) && (on_around != 2 && on_around != 3) {
                    self.toggle(x, y);
                } else if !lights.get(x, y) && (on_around == 3) {
                    self.toggle(x, y);
                }
            }
        }
    }

    pub fn animate2(&mut self) {
        let w = self.width - 1;
        let h = self.height - 1;

        self.animate();
        self.turn_on(0, 0);
        self.turn_on(w, 0);
        self.turn_on(0, h);
        self.turn_on(w, h);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test1() {
        let input = ".#.#.#\n\
                     ...##.\n\
                     #....#\n\
                     ..#...\n\
                     #.#..#\n\
                     ####..";

        let mut lights = super::parse(input);

        lights.animate();
        assert_eq!(11, lights.lights_on());

        lights.animate();
        assert_eq!(8, lights.lights_on());

        lights.animate();
        assert_eq!(4, lights.lights_on());

        lights.animate();
        assert_eq!(4, lights.lights_on());
    }

    #[test]
    fn part2_test1() {
        let input = "##.#.#\n\
                     ...##.\n\
                     #....#\n\
                     ..#...\n\
                     #.#..#\n\
                     ####.#";

        let mut lights = super::parse(input);

        lights.animate2();
        assert_eq!(18, lights.lights_on());

        lights.animate2();
        assert_eq!(18, lights.lights_on());

        lights.animate2();
        assert_eq!(18, lights.lights_on());

        lights.animate2();
        assert_eq!(14, lights.lights_on());

        lights.animate2();
        assert_eq!(17, lights.lights_on());
    }
}
