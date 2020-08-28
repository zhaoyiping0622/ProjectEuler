use num::integer::{sqrt, Roots};

const LIMIT: usize = 28123;
fn isAbundant(num: i32) -> bool {
    (1..=num.sqrt())
        .filter(|x| num % x == 0)
        .map(|x| if num / x != x { num / x + x } else { x })
        .sum::<i32>()
        > num * 2
}
fn main() {
    let mut abundant = Vec::new();
    for i in 2..=LIMIT {
        if isAbundant(i as i32) {
            abundant.push(i);
        }
    }
    let mut can = vec![false; LIMIT + 1];
    for i in abundant.iter() {
        for j in abundant.iter() {
            if i + j > LIMIT {
                break;
            }
            can[i + j] = true;
        }
    }
    println!(
        "{:?}",
        can.into_iter()
            .enumerate()
            .filter(|&x| { !x.1 })
            .map(|(index, _)| index)
            .sum::<usize>()
    );
}
