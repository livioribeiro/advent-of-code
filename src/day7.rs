const INPUT: &'static str = include_str!("data/day7.txt");

pub fn part1() -> u16 {
    calculate_part1(INPUT)
}

pub fn part2() -> u16 {
    calculate_part2(INPUT)
}

fn calculate_part1(input: &str) -> u16 {
    let mut circuit = Circuit::new(input);

    circuit.resolve("a")
}

fn calculate_part2(input: &str) -> u16 {
    let mut circuit = Circuit::new(input);
    circuit.set_gate("b", 3176);

    circuit.resolve("a")
}

use std::collections::HashMap;

pub struct Circuit {
    gates: HashMap<String, Gate>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    pub fn new(input: &str) -> Self {
        let mut circuit: HashMap<String, Gate> = HashMap::new();

        for line in input.lines() {
            let (result, instruction) = parse_instruction(line);
            circuit.insert(result, instruction);
        }

        Circuit {
            gates: circuit,
            cache: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, ident: &str) -> u16 {
        if let Some(value) = self.cache.get(ident) {
            return value.to_owned()
        }

        let instruction = self.gates[ident].clone();

        match instruction {
            Gate::And(lhs, rhs) => {
                let lhs = match lhs {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };
                let rhs = match rhs {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };

                let result = lhs & rhs;
                self.cache.insert(ident.to_owned(), result);

                result
            }
            Gate::Or(lhs, rhs) => {
                let lhs = match lhs {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };
                let rhs = match rhs {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };

                let result = lhs | rhs;
                self.cache.insert(ident.to_owned(), result);

                result
            }
            Gate::Not(rhs) => {
                let rhs = match rhs {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };

                let result = !rhs;
                self.cache.insert(ident.to_owned(), result);

                result
            }
            Gate::LShift(lhs, rhs) => {
                let lhs = match lhs {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };

                let result = lhs << rhs;
                self.cache.insert(ident.to_owned(), result);

                result
            }
            Gate::RShift(lhs, rhs) => {
                let lhs = match lhs {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };

                let result = lhs >> rhs;
                self.cache.insert(ident.to_owned(), result);

                result
            }
            Gate::Signal(val) => {
                let result = match val {
                    Node::Reference(ref ident) => self.resolve(ident),
                    Node::Value(sig) => sig,
                };

                self.cache.insert(ident.to_owned(), result);

                result
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_gate(&mut self, ident: &str, value: u16) {
        self.gates.remove(ident).unwrap();
        self.gates.insert(ident.to_owned(), Gate::Signal(Node::Value(value)));
    }
}

#[derive(Clone)]
enum Node {
    Reference(String),
    Value(u16),
}

#[derive(Clone)]
enum Gate {
    And(Node, Node),
    Or(Node, Node),
    Not(Node),
    RShift(Node, u16),
    LShift(Node, u16),
    Signal(Node),
}

fn parse_instruction(line: &str) -> (String, Gate) {
    let op: Vec<_> = line.split(" -> ").collect();
    let result = op[1];

    let instruction = {
        if line.contains("AND") {
            parse_and(line)
        } else if line.contains("OR") {
            parse_or(line)
        } else if line.contains("NOT") {
            parse_not(line)
        } else if line.contains("LSHIFT") {
            parse_lshift(line)
        } else if line.contains("RSHIFT") {
            parse_rshift(line)
        } else {
            parse_signal(line)
        }
    };

    (result.to_owned(), instruction)
}

fn parse_and(line: &str) -> Gate {
    let instruction: Vec<_> = line.split(" -> ").collect();
    let ops: Vec<_> = instruction[0].split(" AND ").collect();

    let lhs = match ops[0].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[0].to_owned()),
    };

    let rhs = match ops[1].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[1].to_owned()),
    };

    Gate::And(lhs, rhs)
}

fn parse_or(line: &str) -> Gate {
    let instruction: Vec<_> = line.split(" -> ").collect();
    let ops: Vec<_> = instruction[0].split(" OR ").collect();

    let lhs = match ops[0].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[0].to_owned()),
    };

    let rhs = match ops[1].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[1].to_owned()),
    };

    Gate::Or(lhs, rhs)
}

fn parse_not(line: &str) -> Gate {
    let instruction: Vec<_> = line.split(" -> ").collect();
    let ops: Vec<_> = instruction[0].split("NOT ").collect();

    let rhs = match ops[0].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[1].to_owned()),
    };

    Gate::Not(rhs)
}

fn parse_lshift(line: &str) -> Gate {
    let instruction: Vec<_> = line.split(" -> ").collect();
    let ops: Vec<_> = instruction[0].split(" LSHIFT ").collect();

    let lhs = match ops[0].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[0].to_owned()),
    };

    let rhs: u16 = ops[1].parse().expect(format!("Invalid input: {}", ops[1]).as_ref());

    Gate::LShift(lhs, rhs)
}

fn parse_rshift(line: &str) -> Gate {
    let instruction: Vec<_> = line.split(" -> ").collect();
    let ops: Vec<_> = instruction[0].split(" RSHIFT ").collect();

    let lhs = match ops[0].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[0].to_owned()),
    };

    let rhs: u16 = ops[1].parse().expect(format!("Invalid input: {}", ops[1]).as_ref());

    Gate::RShift(lhs, rhs)
}

fn parse_signal(line: &str) -> Gate {
    let ops: Vec<_> = line.split(" -> ").collect();
    let signal = match ops[0].parse::<u16>() {
        Ok(val) => Node::Value(val),
        Err(_) => Node::Reference(ops[0].to_owned()),
    };

    Gate::Signal(signal)
}

#[cfg(test)]
mod tests {
    use super::Circuit;
    #[test]
    fn test1() {
        let input = "123 -> x\n\
                     456 -> y\n\
                     x AND y -> d\n\
                     x OR y -> e\n\
                     x LSHIFT 2 -> f\n\
                     y RSHIFT 2 -> g\n\
                     NOT x -> h\n\
                     NOT y -> i";

        let mut circuit = Circuit::new(input);
        assert_eq!(72, circuit.resolve("d"));
        assert_eq!(507, circuit.resolve("e"));
        assert_eq!(492, circuit.resolve("f"));
        assert_eq!(114, circuit.resolve("g"));
        assert_eq!(65412, circuit.resolve("h"));
        assert_eq!(65079, circuit.resolve("i"));
        assert_eq!(123, circuit.resolve("x"));
        assert_eq!(456, circuit.resolve("y"));
    }
}
