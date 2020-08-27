const LIMIT: i32 = 400_0000;

struct Solution(i32, i32);

impl Iterator for Solution {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 > LIMIT {
            None
        } else {
            let ret = self.1;
            self.1 = self.0;
            self.0 = ret + self.0;
            Some(ret)
        }
    }
}

fn main() {
    let solution = Solution(2, 1);
    println!(
        "{}",
        solution.into_iter().filter(|x| x % 2 == 0).sum::<i32>()
    );
}
