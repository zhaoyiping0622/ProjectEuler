use ProjectEuler::NumberTheory::{get_primes, is_prime};
fn check_two_primes(x: i32, y: i32) -> bool {
    let xx: i64 = format!("{}{}", x, y).parse().unwrap();
    x == y || is_prime(xx)
}
fn dfs(
    deep: i32,
    now: usize,
    nowSum: i32,
    graph: &Vec<Vec<usize>>,
    mut max: i32,
    values: Vec<usize>,
    toSearch: &Vec<usize>,
    primes: &Vec<i32>,
) -> i32 {
    if deep == 5 {
        let ans: Vec<i32> = values.iter().map(|x| primes[*x]).collect();
        println!("{:?}", ans);
        return nowSum;
    }
    for i in (now + 1)..toSearch.len() {
        let tmpSum = nowSum + primes[toSearch[i]];
        if tmpSum < max
            && values
                .iter()
                .all(|x| graph[*x].binary_search(&toSearch[i]).is_ok())
        {
            let mut nValues = values.clone();
            nValues.push(toSearch[i]);
            let nmax = dfs(deep + 1, i, tmpSum, graph, max, nValues, toSearch, primes);
            if nmax < max {
                max = nmax;
            }
        }
    }
    return max;
}
fn main() {
    let up = 10_000;
    let primes = get_primes(up);
    let mut match_primes = vec![Vec::new(); primes.len()];
    for i in 0..primes.len() {
        for j in i + 1..primes.len() {
            if check_two_primes(primes[i], primes[j]) && check_two_primes(primes[j], primes[i]) {
                match_primes[i].push(j);
                // match_primes[j].push(i);
            }
        }
    }
    match_primes.iter_mut().for_each(|x| x.sort());
    // println!("{:?}", match_primes);
    let mut ans = 5 * up as i32;
    for i in 0..match_primes.len() {
        ans = dfs(
            1,
            0,
            primes[i],
            &match_primes,
            ans,
            vec![i],
            &match_primes[i],
            &primes,
        );
    }
    println!("{}", ans);
}
