fn solve(num: i32) -> u32 {
    let mut nums = Vec::new();
    for i in 1.. {
        let mut t = num * i;
        let mut tmp = Vec::new();
        while t != 0 {
            tmp.push(t % 10);
            t /= 10;
        }
        while !tmp.is_empty() {
            nums.push(tmp.pop().unwrap());
        }
        if nums.len() >= 9 {
            break;
        }
    }
    if nums.len() > 9 {
        return 0;
    }
    let mut count = [0; 10];
    for &x in &nums {
        count[x as usize] += 1;
    }
    for i in 1..10 {
        if count[i] != 1 {
            return 0;
        }
    }
    nums.iter().fold(0, |x, &y| 10 * x + y as u32)
}
fn main() {
    println!("{}", (1..100_000).map(|x| solve(x)).max().unwrap());
}
