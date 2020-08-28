fn main() {
    let mut nums = vec![0; 10000];
    for i in 2..10000 {
        for j in 1..i {
            if j * j == i {
                nums[i] += j;
            }
            if j * j >= i {
                break;
            }
            if i % j == 0 {
                nums[i] += j + i / j;
            }
        }
        nums[i] -= i;
    }
    let mut ans = 0;
    for i in 1..nums.len() {
        if nums[i] < nums.len() && i == nums[nums[i]] && i != nums[i] {
            ans += i;
        }
    }
    println!("{}", ans);
}
