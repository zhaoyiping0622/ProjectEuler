fn getCount(beginDay: i32) -> i32 {
    if beginDay >= 5 {
        5
    } else {
        4
    }
}
fn main() {
    let mut year = 1901;
    let mut beginDay = 2;
    let mut ans = 0;
    let normalDays = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leapDays = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    while year < 2001 {
        let days: &[i32; 12];
        if year % 400 == 0 || (year % 100 != 0 && year % 4 == 0) {
            days = &leapDays;
        } else {
            days = &normalDays;
        }
        for x in days {
            beginDay = (beginDay + x) % 7;
            if beginDay == 0 {
                ans += 1;
            }
        }
        year += 1;
    }
    println!("{}", ans);
}
