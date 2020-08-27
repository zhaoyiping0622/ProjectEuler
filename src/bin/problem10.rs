const LIMIT: usize = 200_0000;

fn main() {
    let mut isPrime = vec![true; LIMIT];
    let mut prime = Vec::new();
    isPrime[1] = false;
    for i in 1..LIMIT {
        if isPrime[i] {
            prime.push(i);
        }
        for &j in prime.iter() {
            if j * i >= LIMIT {
                break;
            }
            isPrime[i * j] = false;
            if j % i == 0 {
                break;
            }
        }
    }
    println!("{}", prime.into_iter().sum::<usize>());
}
