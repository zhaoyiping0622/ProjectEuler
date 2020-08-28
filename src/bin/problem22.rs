use std::fs;
fn main() {
    let filename = "resources/p022_names.txt";
    let input = fs::read_to_string(filename).unwrap();
    let mut input = input.split(",").collect::<Vec<&str>>();
    input.sort();
    let input = input
        .iter()
        .map(|&x| {
            x.chars()
                .filter(|x| *x != '\"')
                .map(|x| (x as u8 - 'A' as u8 + 1) as i32)
                .sum::<i32>()
        })
        .enumerate()
        .map(|(index, x)| (index + 1) * x as usize)
        .sum::<usize>();
    println!("{:?}", input);
}
