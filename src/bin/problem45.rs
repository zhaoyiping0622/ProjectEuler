fn main() {
    let a = 286;
    let b = 166;
    let c = 144;
    let t = |x| x * (x + 1) / 2;
    let p = |x| x * (3 * x - 1) / 2;
    let h = |x| x * (2 * x - 1);
    let mut all: [(u64, Box<dyn Fn(u64) -> u64>); 3] =
        [(a, Box::new(t)), (b, Box::new(p)), (c, Box::new(h))];
    println!(
        "{}",
        loop {
            let nums: Vec<u64> = all.iter().map(|x| x.1(x.0)).collect();
            if nums[0] == nums[1] && nums[1] == nums[2] {
                break nums[0];
            }
            all.iter_mut()
                .min_by(|x, y| x.1(x.0).cmp(&y.1(y.0)))
                .unwrap()
                .0 += 1;
        }
    );
}
