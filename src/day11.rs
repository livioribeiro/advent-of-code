const INPUT_PART1: &'static [u8; 8] = b"vzbxkghb";
const INPUT_PART2: &'static [u8; 8] = b"vzbxxzaa";

const A: u8 = 'a' as u8;
const Z: u8 = 'z' as u8;

// forbidden characters
const I: u8 = 'i' as u8;
const L: u8 = 'l' as u8;
const O: u8 = 'o' as u8;

const FORBIDDEN: [u8; 3] = [I, L, O];

pub fn part1() -> String {
    calculate(INPUT_PART1)
}

pub fn part2() -> String {
    calculate(INPUT_PART2)
}

fn calculate(input: &[u8; 8]) -> String {
    let mut input = input.to_owned();

    while !is_valid(&input) {
        if !check_forbidden_chars(&input) {
            increment_forbidden_char(&mut input);
            continue
        }

        increment(&mut input);

        if input == [A, A, A, A, A, A, A, A] {
            panic!("No valid password found");
        }
    }

    String::from_utf8(Vec::from(input.as_ref())).expect("Could not convert from utf-8")
}

fn is_valid(pass: &[u8; 8]) -> bool {
    check_increasing_sequence(pass) && check_forbidden_chars(pass) && check_pairs(pass)
}

fn increment(pass: &mut [u8; 8]) {
    for i in (0..8).rev() {
        if pass[i] == Z {
            if i == 0 {
                for j in 0..8 { pass[j] = A }
            }
            pass[i] = A;
        } else {
            pass[i] += 1;
            break;
        }
    }
}

fn increment_forbidden_char(pass: &mut [u8; 8]) {
    for i in (0..8).rev() {
        if FORBIDDEN.contains(&pass[i]) {
            pass[i] += 1;
            for j in (i + 1)..8 { pass[j] = A }
            break;
        }
    }
}

fn check_increasing_sequence(pass: &[u8; 8]) -> bool {
    for i in 0..6 {
        if pass[i + 1] == (pass[i] + 1) && pass[i + 2] == (pass[i] + 2) {
            return true
        }
    }

    false
}

fn check_forbidden_chars(pass: &[u8; 8]) -> bool {
    for c in pass.iter() {
        if FORBIDDEN.contains(c) {
            return false
        }
    }

    true
}

fn check_pairs(pass: &[u8; 8]) -> bool {
    let mut pair_1 = 0;
    let mut pair_2 = 0;

    let mut second_loop_start = 0;

    for i in 0..5 {
        if pass[i] == pass[i + 1] {
            pair_1 = pass[i];
            second_loop_start = i + 2;
            break
        }
    }

    if pair_1 == 0 {
        return false
    }

    for i in second_loop_start..7 {
        if pass[i] == pass[i + 1] {
            pair_2 = pass[i];
            break
        }
    }

    if pair_2 == 0 {
        return false
    }

    pair_1 != pair_2
}

#[cfg(test)]
mod tests {
    use super::{A, Z, I, L, O};

    const B: u8 = 'b' as u8;

    #[test]
    fn increment_simple() {
        let mut input = [A, A, A, A, A, A, A, A];
        super::increment(&mut input);

        assert_eq!([A, A, A, A, A, A, A, B], input);
    }

    #[test]
    fn increment_not_so_simple() {
        let mut input = [A, A, A, A, A, A, A, Z];
        super::increment(&mut input);

        assert_eq!([A, A, A, A, A, A, B, A], input);
    }

    #[test]
    fn increment_complicated() {
        let mut input = [Z, Z, Z, Z, Z, Z, Z, Z];
        super::increment(&mut input);

        assert_eq!([A, A, A, A, A, A, A, A], input);
    }

    #[test]
    fn has_increasing_sequence_at_beginning() {
        let input = [A, A + 1, A + 2, A, A, A, A, A];
        assert!(super::check_increasing_sequence(&input));
    }

    #[test]
    fn has_increasing_sequence_at_end() {
        let input = [A, A, A, A, A, A, A + 1, A + 2];
        assert!(super::check_increasing_sequence(&input));
    }

    #[test]
    fn has_increasing_sequence_at_middle() {
        let input = [A, A, A, A + 1, A + 2, A, A, A];
        assert!(super::check_increasing_sequence(&input));
    }

    #[test]
    fn doesnt_has_increasing_sequence_should_fail() {
        let input = [A, Z, A, Z, A, Z, A, Z];
        assert!( ! super::check_increasing_sequence(&input));
    }

    #[test]
    fn has_forbidden_chars_should_fail() {
        let input = [A, I, A, A, A, A, A, A];
        assert!( ! super::check_forbidden_chars(&input));

        let input = [A, A, A, L, A, A, A, A];
        assert!( ! super::check_forbidden_chars(&input));

        let input = [A, A, A, A, A, A, O, A];
        assert!( ! super::check_forbidden_chars(&input));
    }

    #[test]
    fn has_multiple_forbdden_chars_should_fail() {
        let input = [A, I, A, L, A, O, A, A];
        assert!( ! super::check_forbidden_chars(&input));
    }

    #[test]
    fn doesnt_has_forbidden_chars() {
        let input = [A, Z, A, Z, A, Z, A, Z];
        assert!(super::check_forbidden_chars(&input));
    }

    #[test]
    fn has_two_different_pairs_at_begenning() {
        let input = [A, A, B, B, A, Z, A, Z];
        assert!(super::check_pairs(&input));
    }

    #[test]
    fn has_two_different_pairs_at_end() {
        let input = [A, Z, A, Z, A, A, B, B];
        assert!(super::check_pairs(&input));
    }

    #[test]
    fn has_two_different_pairs_at_middle() {
        let input = [A, Z, A, A, B, B, A, Z];
        assert!(super::check_pairs(&input));
    }

    #[test]
    fn has_two_different_pairs_at_sides() {
        let input = [A, A, Z, Z, Z, Z, B, B];
        assert!(super::check_pairs(&input));
    }

    #[test]
    fn doesnt_has_two_different_pairs_should_fail() {
        let input = [A, Z, A, Z, A, Z, A, Z];
        assert!( ! super::check_pairs(&input));
    }

    #[test]
    fn has_two_equal_pairs_should_fail() {
        let input = [A, A, A, Z, A, A, A, Z];
        assert!( ! super::check_pairs(&input));
    }

    #[test]
    fn has_only_one_pair_should_fail() {
        let input = [A, A, Z, A, Z, A, Z, A];
        assert!( ! super::check_pairs(&input));
    }

    #[test]
    fn increment_forbidden() {
        let mut input = [A, A, A, I, B, Z, B, B];
        super::increment_forbidden_char(&mut input);

        assert_eq!([A, A, A, I + 1, A, A, A, A], input);
    }
}
