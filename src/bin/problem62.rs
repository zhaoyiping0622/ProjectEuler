use std::collections::HashMap;

use itertools::Itertools;
fn main() {
    let mut map = HashMap::new();
    let mut flag = false;
    let mut prelen = 0;
    for i in 1 as u64.. {
        let x = i * i * i;
        let s: String = x.to_string().chars().sorted().collect();
        if flag && s.len() != prelen {
            break;
        }
        prelen = s.len();
        let t = map.entry(s).or_insert(Vec::new());
        t.push(i);
        if t.len() == 5 {
            flag = true;
        }
    }
    println!(
        "{}",
        map.iter()
            .filter(|x| x.1.len() == 5)
            .map(|x| x.1[0])
            .min()
            .map(|x| x * x * x)
            .unwrap()
    );
}
