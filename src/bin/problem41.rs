use itertools::Itertools;
use num::integer::Roots;

fn isPrime(num: i32) -> bool {
    for i in 2..num.sqrt() + 1 {
        if num % i == 0 {
            return false;
        }
    }
    true
}
fn solve(num: i32) -> i32 {
    (1..=num)
        .permutations(num as usize)
        .map(|x| x.iter().fold(0, |x, y| 10 * x + y))
        .filter(|&x| isPrime(x))
        .max()
        .unwrap_or(0)
}
fn main() {
    println!("{}", (1..=9).map(|x| solve(x)).max().unwrap());
}
