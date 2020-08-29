use std::fs;
fn main() {
    //
    let input = fs::read_to_string("resources/p042_words.txt").unwrap();
    let mut input: Vec<i32> = input
        .split(",")
        .map(|x| {
            x.chars()
                .filter(|x| x.is_alphabetic())
                .fold(0, |x, y| x + ((y as u8 - ('A' as u8) + 1) as i32))
        })
        .collect();
    input.sort();
    let mut sum = 0;
    let mut now = 1;
    let mut count = 0;
    for x in input {
        while sum < x {
            sum += now;
            now += 1;
        }
        if x == sum {
            count += 1;
        }
    }
    println!("{}", count);
}
