use itertools::Itertools;
use num::traits::Pow;
use std::collections::HashSet;

fn getNum(nums: &Vec<i32>) -> i32 {
    nums.iter().map(|&x| x.pow(5)).sum()
}

fn judge(nums: &Vec<i32>) -> bool {
    match nums.iter().map(|x| x.to_string()).join("").parse::<i32>() {
        Err(..) => false,
        Ok(x) => x == getNum(nums),
    }
}
fn dfs(deep: i32, nums: &mut Vec<i32>, ans: &mut Vec<i32>) -> i32 {
    if deep == -1 {
        return 0;
    }
    let mut ret = 0;
    if judge(nums) {
        ans.push(getNum(nums));
        ret += getNum(nums);
    }
    for i in 0..10 {
        nums.push(i);
        ret += dfs(deep - 1, nums, ans);
        nums.pop();
    }
    ret
}
fn main() {
    let mut ans = Vec::new();
    dfs(6, &mut Vec::new(), &mut ans);
    let ans = {
        let mut tmp = HashSet::new();
        ans.iter().for_each(|&x| {
            tmp.insert(x);
        });
        tmp
    };
    println!("{}", ans.iter().sum::<i32>() - 1);
}
