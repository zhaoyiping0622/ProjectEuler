use itertools::Itertools;

fn f3(n: i32) -> i32 {
    n * (n + 1) / 2
}
fn f4(n: i32) -> i32 {
    n * n
}
fn f5(n: i32) -> i32 {
    n * (3 * n - 1) / 2
}
fn f6(n: i32) -> i32 {
    n * (2 * n - 1)
}
fn f7(n: i32) -> i32 {
    n * (5 * n - 3) / 2
}
fn f8(n: i32) -> i32 {
    n * (3 * n - 2)
}
fn dfs(deep: usize, values: Vec<(i32, i32)>, fs: &Vec<Vec<(i32, i32)>>) -> bool {
    if deep == fs.len() {
        if values[0].1 / 100 == values.last().unwrap().1 % 100 {
            let values: Vec<i32> = values.into_iter().map(|x| x.1).collect();
            let sum: i32 = values.iter().sum();
            println!("{:?} sum={}", values, sum);
            return true;
        }
    } else {
        for x in fs[deep].iter() {
            if values.last().unwrap().1 % 100 == x.1 / 100 && values.iter().all(|y| y.0 != x.0) {
                let mut nValues = values.clone();
                nValues.push(*x);
                if dfs(deep + 1, nValues, fs) {
                    return true;
                }
            }
        }
    }
    false
}
fn main() {
    let fs: Vec<&dyn Fn(i32) -> i32> = vec![&f3, &f4, &f5, &f6, &f7, &f8];
    for fs in fs.iter().permutations(fs.len()) {
        let values: Vec<Vec<(i32, i32)>> = fs
            .iter()
            .map(|f| {
                (0..)
                    .map(|x| (x, f(x)))
                    .take_while(|x| x.1 < 10000)
                    .filter(|x| x.1 >= 1000)
                    .collect()
            })
            .collect();
        for x in values[0].iter() {
            dfs(1, vec![*x], &values);
        }
    }
}
