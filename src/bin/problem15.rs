fn main() {
    let mut dp = vec![vec![0u64; 21]; 21];
    dp[0][0] = 1;
    for i in 0..21 {
        for j in 0..21 {
            if i != 0 {
                dp[i][j] += dp[i - 1][j];
            }
            if j != 0 {
                dp[i][j] += dp[i][j - 1];
            }
        }
    }
    println!("{}", dp.last().unwrap().last().unwrap());
}
