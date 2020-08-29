use std::collections::LinkedList;

const LIMIT: usize = 1000_0000 + 1;
fn getNum(nums: &LinkedList<i32>) -> i32 {
    nums.iter().fold(0, |x, y| x * 10 + y)
}
fn judge(num: i32, isPrime: &Vec<bool>) -> bool {
    let mut num = num;
    let mut nums = Vec::new();
    while num != 0 {
        nums.push(num % 10);
        num /= 10;
    }
    let mut nums = nums.iter().rev().map(|&x| x).collect::<LinkedList<i32>>();
    for _i in 0..nums.len() {
        let num = getNum(&nums) as usize;
        if !isPrime[num] {
            return false;
        }
        let num = nums.pop_front().unwrap();
        nums.push_back(num);
    }
    true
}
fn main() {
    let mut isPrime = vec![true; LIMIT];
    let mut primes = Vec::new();
    isPrime[1] = false;
    for i in 2..LIMIT {
        if isPrime[i] {
            primes.push(i);
        }
        for x in &primes {
            if x * i >= LIMIT {
                break;
            }
            isPrime[x * i] = false;
            if x % i == 0 {
                break;
            }
        }
    }
    println!(
        "{}",
        primes
            .iter()
            .take(LIMIT / 10)
            .filter(|&&x| judge(x as i32, &isPrime))
            .map(|&x| x)
            .count()
    );
}
