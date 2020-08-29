fn judge(num: i32) -> bool {
    if num.to_string() != num.to_string().chars().rev().collect::<String>() {
        return false;
    }
    let mut nums = Vec::new();
    let mut num = num;
    while num != 0 {
        nums.push(num % 2);
        num /= 2;
    }
    nums == nums.iter().rev().map(|&x| x).collect::<Vec<i32>>()
}
fn main() {
    println!("{}", (1..=100_0000).filter(|&x| judge(x)).sum::<i32>());
}
