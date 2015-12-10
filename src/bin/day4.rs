extern crate md5;
extern crate rustc_serialize;

use rustc_serialize::hex::ToHex;

fn main() {
    let input = "iwrupvqb";

    for i in 0..u32::max_value() {
        let input = format!("{}{}", input, i);
        let digest = md5::compute(input.as_bytes());
        let digest = digest.to_hex();

        let first_five: Vec<char> = digest.chars().take(5).collect();

        if first_five == ['0', '0', '0', '0', '0'] {
            println!("{}", i);
            break;
        }
    }
}
