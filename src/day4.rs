// thanks to https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
// for the optimizations

use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "iwrupvqb";

pub fn part1() -> u64 {
    calculate_part1(INPUT.as_bytes())
}

pub fn part2() -> u64 {
    calculate_part2(INPUT.as_bytes())
}

fn calculate_part1(input: &[u8]) -> u64 {
    let mut hasher = Md5::new();

    for i in 0..::std::u64::MAX {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_five = (output[0] as i32)  + (output[1] as i32) + ((output[2] >> 4) as i32);

        if first_five == 0 {
            return i
        }

        hasher.reset();
    }

    panic!("The mining operation has failed!");
}

fn calculate_part2(input: &[u8]) -> u64 {
    let mut hasher = Md5::new();

    for i in 0..::std::u64::MAX {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_five = (output[0] as i32)  + (output[1] as i32) + (output[2] as i32);

        if first_five == 0 {
            return i
        }

        hasher.reset();
    }

    panic!("The mining operation has failed!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(609043, super::calculate_part1("abcdef".as_bytes()));
    }

    #[test]
    fn test2() {
        assert_eq!(1048970, super::calculate_part1("pqrstuv".as_bytes()));
    }
}
