fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    println!(
        "{}",
        square((1..=100).sum::<i32>()) - (1..=100).map(square).sum::<i32>()
    )
}
