use num::integer::Roots;

fn factorCount(num: i32) -> i32 {
    let mut num = num;
    let mut count = 0;
    for i in 2..num.sqrt() + 1 {
        if num % i == 0 {
            count += 1;
            while num % i == 0 {
                num /= i;
            }
        }
        if num == 1 {
            break;
        }
    }
    if num != 1 {
        count += 1;
    }
    count
}
fn main() {
    let nums: Vec<i32> = (1..1000000)
        .map(|value| (value, factorCount(value)))
        .filter(|x| x.1 == 4)
        .map(|x| x.0)
        .collect();
    for i in 0..nums.len() {
        if i + 3 < nums.len() && nums[i + 3] == nums[i] + 3 {
            println!("{}", nums[i]);
            return;
        }
    }
    unreachable!();
}
