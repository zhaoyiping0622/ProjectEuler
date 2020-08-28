use itertools::Itertools;
fn main() {
    println!(
        "{}",
        (0..=9)
            .permutations(10)
            .skip(100_0000 - 1)
            .next()
            .unwrap()
            .iter()
            .map(|x| x.to_string())
            .join("")
    );
}
