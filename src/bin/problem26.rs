use std::collections::HashMap;
fn getLen(num: i32) -> i32 {
    let mut map = HashMap::new();
    map.insert(1, 0);
    let mut now = 1;
    for i in 1.. {
        now = now * 10 % num;
        if now == 0 {
            break;
        }
        match map.get(&now) {
            None => {
                map.insert(now, i);
            }
            Some(x) => {
                return i - x;
            }
        }
    }
    map.len() as i32
}
fn main() {
    println!(
        "{:?}",
        (2..=1000)
            .enumerate()
            .map(|(index, x)| (index + 2, getLen(x)))
            .max_by(|x, y| x.1.cmp(&y.1))
            .unwrap()
            .0
    );
}
