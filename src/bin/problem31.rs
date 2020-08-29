fn main() {
    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];
    let mut dp = vec![0; 201];
    dp[0] = 1;
    for coin in &coins {
        for i in *coin..201 {
            dp[i] += dp[i - coin];
        }
    }
    println!("{}", dp.last().unwrap());
}
