// Optimized using https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    let input = "iwrupvqb".as_bytes();

    let mut hasher = Md5::new();

    for i in 0..std::u64::MAX {
        hasher.input(input);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        let first_five = (output[0] as i32)  + (output[1] as i32) + (output[2] as i32);

        if first_five == 0 {
            println!("{}", i);
            break;
        }

        hasher.reset();
    }
}
