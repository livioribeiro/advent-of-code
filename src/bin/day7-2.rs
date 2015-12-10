#[path="day7.rs"]
mod day7;

use day7::Circuit;

fn main() {
    let input = include_str!("day7.txt");

    let mut circuit = Circuit::new(input);
    circuit.set_gate("b", 3176);

    println!("{}", circuit.resolve("a"));
}
