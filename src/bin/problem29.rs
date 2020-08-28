use num::BigInt;
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    for i in 2..=100 {
        for j in 2..=100 {
            let num = BigInt::from(i);
            set.insert(num.pow(j));
        }
    }
    println!("{}", set.len());
}
