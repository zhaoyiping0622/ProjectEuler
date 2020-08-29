const LIMIT: usize = 1000_0000 + 1;
fn judge(num: i32, isPrime: &Vec<bool>) -> bool {
    let mut num = num;
    let mut top = 1000_0000;
    while top != 1 {
        if !isPrime[(num % top) as usize] {
            return false;
        }
        top /= 10;
    }
    while num != 0 {
        if !isPrime[num as usize] {
            return false;
        }
        num /= 10;
    }
    true
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
    println!(
        "{:?}",
        primes
            .iter()
            .filter(|&&x| judge(x as i32, &isPrime))
            .map(|&x| x)
            .sum::<usize>()
            - 2
            - 3
            - 5
            - 7
    );
}
