fn getVecOfNum(num: i32) -> Vec<i32> {
    let mut ret = Vec::new();
    let mut num = num;
    while num != 0 {
        ret.push(num % 10);
        num /= 10;
    }
    ret.into_iter().rev().collect()
}

fn judge(num: i32) -> bool {
    let mut now = getVecOfNum(num * 2);
    for i in 3..=6 {
        let mut tmp = getVecOfNum(num * i);
        now.sort();
        tmp.sort();
        if now != tmp {
            return false;
        }
    }
    true
}

fn main() {
    for i in 1.. {
        if judge(i) {
            println!("{}", i);
            break;
        }
    }
}
