use std::collections::HashMap;

fn dfs(x: usize, nums: &mut HashMap<usize, i32>) -> i32 {
    match nums.get(&x) {
        None => {
            if x % 2 == 0 {
                let num = dfs(x / 2, nums) + 1;
                nums.insert(x, num);
            } else {
                let num = dfs(x * 3 + 1, nums) + 1;
                nums.insert(x, num);
            }
            *nums.get(&x).unwrap()
        }
        Some(&x) => x,
    }
}

fn main() {
    let mut nums = HashMap::new();
    nums.insert(1, 1);
    for i in 1..100_0000 {
        dfs(i, &mut nums);
    }
    println!(
        "{:?}",
        nums.iter()
            .filter(|&x| *(x.1) <= 100_0000)
            .max_by(|&x, &y| (x.1).cmp(y.1))
            .unwrap()
    );
}
