use itertools::Itertools;
use std::collections::HashSet;

fn getNum(nums: &Vec<i32>) -> i32 {
    let mut factor = Vec::new();
    factor.push(1);
    for i in 1..10 {
        factor.push(factor.last().unwrap() * i);
    }
    nums.iter().map(|&x| factor[x as usize]).sum()
}

fn judge(nums: &Vec<i32>) -> bool {
    nums.len() != 0
        && nums
            .iter()
            .map(|&x| x.to_string())
            .join("")
            .parse::<i32>()
            .unwrap()
            == getNum(nums)
}

fn dfs(deep: i32, nums: &mut Vec<i32>, ans: &mut HashSet<i32>) {
    if judge(nums) {
        ans.insert(getNum(nums));
    }
    if deep == 0 {
        return;
    }
    if nums.len() == 0 {
        for i in 1..10 {
            nums.push(i);
            dfs(deep - 1, nums, ans);
            nums.pop();
        }
    } else {
        for i in 0..10 {
            nums.push(i);
            dfs(deep - 1, nums, ans);
            nums.pop();
        }
    }
}

fn main() {
    let mut set = HashSet::new();
    dfs(6, &mut Vec::new(), &mut set);
    println!("{}", set.iter().sum::<i32>() - 1 - 2);
}
