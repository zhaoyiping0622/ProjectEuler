fn main() {
    for i in 1..1000 {
        for j in 1..1000 - i {
            let k = 1000 - i - j;
            if i * i + j * j == k * k {
                println!("{}", i * j * k);
            }
        }
    }
}
