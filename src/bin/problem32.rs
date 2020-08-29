use itertools::Itertools;
use std::collections::HashSet;

fn to_i32(nums: &Vec<i32>) -> i32 {
    nums.into_iter().fold(0, |x, &y| 10 * x + y)
}

fn cal(nums: &Vec<i32>) -> HashSet<i32> {
    let mut a = Vec::new();
    let mut ret = HashSet::new();
    for i in 0..nums.len() {
        a.push(nums[i]);
        for j in i + 1..nums.len() {
            let mut b = Vec::new();
            for t in i + 1..j + 1 {
                b.push(nums[t]);
            }
            let mut c = nums.iter().skip(j + 1).map(|&x| x).collect();
            if to_i32(&a) * to_i32(&b) == to_i32(&c) {
                ret.insert(to_i32(&c));
            }
        }
    }
    ret
}

fn main() {
    let mut set = HashSet::new();
    (1..=9).permutations(9).for_each(|x| {
        let tmp = cal(&x);
        for x in tmp {
            set.insert(x);
        }
    });
    println!("{}", set.iter().sum::<i32>());
}
