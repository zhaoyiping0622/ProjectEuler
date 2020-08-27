fn pd(num: i32) -> bool {
    let s = num.to_string();
    s.chars().rev().collect::<String>() == s
}

fn main() {
    let mut ans = 0;
    for i in 100..1000 {
        for j in 100..1000 {
            if pd(i * j) {
                ans = std::cmp::max(ans, i * j);
            }
        }
    }
    println!("{}", ans);
}
