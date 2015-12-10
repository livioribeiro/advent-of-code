fn main() {
    let input = include_str!("day5.txt");

    let mut total = 0;

    for line in input.lines() {
        if two_letters_twice(line) && two_with_one_between(line) {
            total += 1;
        }
    }

    println!("{}", total);
}

fn two_letters_twice(line: &str) -> bool {
    let chars: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(1).collect();

    let size = chars.len();

    let mut i = 0;
    for (c1, c2) in chars.iter().zip(chars2.iter()) {
        let j = i + 2;

        if j > size - 1 {
            break;
        }

        for n in j..(size - 1) {
            if &chars[n] == c1 && &chars[n + 1] == c2 {
                return true
            }
        }

        i += 1;
    }

    false
}

fn two_with_one_between(line: &str) -> bool {
    let chars1: Vec<_> = line.chars().collect();
    let chars2: Vec<_> = line.chars().skip(2).collect();

    for (c1, c2) in chars1.iter().zip(chars2.iter()) {
        if c1 == c2 {
            return true
        }
    }

    false
}
