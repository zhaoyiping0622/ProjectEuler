const LIMIT: usize = 3000_000;
fn getCount(a: i32, b: i32, isPrime: &Vec<bool>) -> i32 {
    let cal = |n: i32| -> i32 { n * n + a * n + b };
    for i in 0.. {
        let t = cal(i);
        if t < 0 {
            return 0;
        }
        if t as usize > isPrime.len() {
            panic!("limit is too small");
        }
        if !isPrime[t as usize] {
            return i;
        }
    }
    0
}
fn main() {
    let mut isPrime = vec![true; LIMIT];
    let mut primes = Vec::new();
    isPrime[1] = false;
    isPrime[0] = false;
    for i in 2..LIMIT {
        if isPrime[i] {
            primes.push(i);
        }
        for x in &primes {
            if x * i >= LIMIT {
                break;
            }
            isPrime[x * i] = false;
            if x % i == 0 {
                break;
            }
        }
    }
    let mut ans = 0;
    let mut count = 0;
    for i in -1000..1000 {
        for j in -1000..1000 {
            let newCount = getCount(i, j, &isPrime);
            if newCount > count {
                count = newCount;
                ans = i * j;
            }
        }
    }
    println!("{}", ans);
}
