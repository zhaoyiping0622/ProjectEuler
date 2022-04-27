use std::fs;
fn valid(x: u8) -> bool {
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ,./<>?;':\"[]{}!-() @#$%^&*()=+_"
        .contains(x as char)
}
fn encrypt(password: &Vec<u8>, input: &Vec<u8>) -> Vec<u8> {
    input
        .iter()
        .enumerate()
        .map(|(x, y)| password[x % password.len()] ^ *y)
        .collect()
}
fn check(password: &Vec<u8>, input: &Vec<u8>) -> bool {
    encrypt(password, input).into_iter().all(valid)
}
fn main() {
    let filename = "resources/p059_cipher.txt";
    let input: Vec<u8> = fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", &input);
    let rg = 'a' as u8..='z' as u8;
    for x in rg.clone() {
        for y in rg.clone() {
            for z in rg.clone() {
                let password = vec![x, y, z];
                if check(&password, &input) {
                    let ans: u32 = encrypt(&password, &input)
                        .into_iter()
                        .map(|x| x as u32)
                        .sum();
                    println!("{}", ans)
                }
            }
        }
    }
}
