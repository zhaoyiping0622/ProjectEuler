use std::collections::HashSet;

use ProjectEuler::NumberTheory::get_primes;

const LIMIT: usize = 1_000_000;

fn getNumber(nums: &Vec<i32>) -> i32 {
    nums.iter().fold(0, |x, y| x * 10 + y)
}

fn getVecOfNumber(num: i32) -> Vec<i32> {
    let mut num = num;
    let mut ret = Vec::new();
    while num != 0 {
        ret.push(num % 10);
        num /= 10;
    }
    ret.iter().rev().map(|&x| x).collect()
}

fn dfs(deep: i32, nums: &mut Vec<i32>, isPrime: &Vec<bool>, head: bool) {
    if deep == 0 {
        let change = nums
            .iter()
            .enumerate()
            .filter(|&(index, &x)| x == 10)
            .map(|(index, x)| index)
            .collect::<Vec<usize>>();
        if change.len() == 0 {
            return;
        }
        let mut nums = nums.clone();
        let begin = if change[0] == 0 { 1 } else { 0 };
        let mut all = Vec::new();
        for i in begin..10 {
            change.iter().for_each(|&x| nums[x] = i);
            let num = getNumber(&nums);
            if isPrime[num as usize] {
                all.push(num);
            }
        }
        if all.len() == 8 {
            println!("{:?}", all);
        }
        return;
    }
    let begin = if head { 1 } else { 0 };
    for i in begin..=10 {
        nums.push(i);
        dfs(deep - 1, nums, isPrime, false);
        nums.pop();
    }
}

fn main() {
    let primes = get_primes(LIMIT);
    let isPrimes = {
        let mut tmp = vec![false; LIMIT];
        primes.iter().for_each(|&x| tmp[x as usize] = true);
        tmp
    };
    dfs(6, &mut Vec::new(), &isPrimes, true);
}
