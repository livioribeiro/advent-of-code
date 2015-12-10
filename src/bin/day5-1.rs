fn main() {
    let input = include_str!("day5.txt");

    let mut total = 0;

    for line in input.lines() {
        if three_vowels(line) && two_in_a_row(line) && !forbidden(line) {
            total += 1;
        }
    }

    println!("{}", total);
}

fn three_vowels(line: &str) -> bool {
    let mut total = 0;

    for c in line.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
            total += 1;
        }
    }

    total >= 3
}

fn two_in_a_row(line: &str) -> bool {
    let chars1: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(1).collect();

    for (c1, c2) in chars1.iter().zip(chars2.iter()) {
        if c1 == c2 {
            return true
        }
    }

    false
}

fn forbidden(line: &str) -> bool {
    line.find("ab")
        .or(line.find("cd"))
        .or(line.find("pq"))
        .or(line.find("xy"))
        .is_some()
}
