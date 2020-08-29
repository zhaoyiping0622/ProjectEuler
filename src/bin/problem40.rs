fn main() {
    let mut s = String::new();
    for i in 1.. {
        s.push_str(i.to_string().as_str());
        if s.len() > 1000000 {
            break;
        }
    }
    println!(
        "{}",
        [
            s.bytes().next().unwrap(),
            s.bytes().skip(10 - 1).next().unwrap(),
            s.bytes().skip(100 - 1).next().unwrap(),
            s.bytes().skip(1000 - 1).next().unwrap(),
            s.bytes().skip(10000 - 1).next().unwrap(),
            s.bytes().skip(100000 - 1).next().unwrap(),
            s.bytes().skip(1000000 - 1).next().unwrap(),
        ]
        .iter()
        .map(|x| x - ('0' as u8))
        .fold(1, |x, y| x * (y as i32))
    )
}
