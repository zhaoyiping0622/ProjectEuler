fn solve(num: i32) -> i32 {
    let mut count = 0;
    for i in 1..num {
        for j in i + 1..num - i {
            let k = num - i - j;
            if i * i + j * j == k * k {
                count += 1;
            }
        }
    }
    count
}
fn main() {
    println!(
        "{}",
        (12..=1000)
            .enumerate()
            .map(|x| (x.0, solve(x.1)))
            .max_by(|&x, &y| x.1.cmp(&y.1))
            .unwrap()
            .0
            + 12
    );
}
