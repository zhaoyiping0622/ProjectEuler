use num::{BigInt, One};
use std::str::FromStr;

fn main() {
    let mut value = (BigInt::one(), BigInt::one());
    let limit = {
        let mut v = Vec::new();
        v.push("1");
        for _i in 0..999 {
            v.push("0");
        }
        BigInt::from_str(&v.join("")).unwrap()
    };
    let mut count = 2;
    loop {
        count += 1;
        value = ((&value.0 + value.1), value.0);
        if value.0.ge(&limit) {
            break;
        }
    }
    println!("{}", count);
}
