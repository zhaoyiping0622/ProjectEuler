use itertools::Itertools;
use std::collections::HashSet;
use ProjectEuler::NumberTheory::get_primes;

fn main() {
    let primes = get_primes(10_000);
    let mut isPrime = vec![false; 10_000];
    primes.into_iter().for_each(|x| isPrime[x as usize] = true);
    (1000..10000).for_each(|x| {
        if isPrime[x] {
            let mut nums = Vec::new();
            let mut t = x;
            for _i in 0..4 {
                nums.push(t % 10);
                t /= 10;
            }
            let mut count = vec![x];
            for v in nums.iter().permutations(4) {
                let num = v.into_iter().fold(0, |x, y| x * 10 + y);
                if num > 1000 && isPrime[num] && num > x {
                    count.push(num)
                }
            }
            let count = {
                let mut tmp = HashSet::new();
                for x in count {
                    tmp.insert(x);
                }
                tmp
            };
            for tmp in &count {
                if *tmp == x {
                    continue;
                }
                if let Some(t) = count.get(&(2 * tmp - x)) {
                    println!("{} {} {}", x, tmp, t);
                }
            }
        }
    })
}
