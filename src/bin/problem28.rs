const LIMIT: i32 = 1001 * 1001;
fn main() {
    let mut sum = 1;
    let mut now = 1;
    let mut delta = 2;
    while now < LIMIT {
        for i in 0..4 {
            now += delta;
            sum += now;
        }
        delta += 2;
    }
    println!("{}", sum);
}
