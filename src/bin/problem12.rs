fn main() {
    let mut value = 0;
    for i in 1.. {
        value += i;
        // println!("shit {}", i);
        let mut divisior = 0;
        for j in 1u64.. {
            if j * j > value {
                break;
            }
            if j * j == value {
                divisior += 1;
                break;
            }
            if value % j == 0 {
                divisior += 2;
            }
        }
        if divisior >= 500 {
            println!("{}", value);
            break;
        }
    }
}
