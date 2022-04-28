use num::{traits::Pow, BigUint};
fn cal(x: i32) -> i32 {
    (1..10)
        .map(|y| BigUint::new(vec![y]).pow(x as u32).to_string().len())
        .filter(|y| *y as i32 == x)
        .count() as i32
}
fn main() {
    println!(
        "{}",
        (1..)
            .take_while(|x| BigUint::new(vec![9]).pow(*x as u32).to_string().len() >= *x as usize)
            .map(cal)
            .sum::<i32>()
    );
}
