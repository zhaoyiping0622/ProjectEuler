use std::collections::HashMap;

fn tran(num: i32) -> String {
    let mut map = HashMap::new();
    map.insert(0, "");
    map.insert(1, "one");
    map.insert(2, "two");
    map.insert(3, "three");
    map.insert(4, "four");
    map.insert(5, "five");
    map.insert(6, "six");
    map.insert(7, "seven");
    map.insert(8, "eight");
    map.insert(9, "nine");
    map.insert(10, "ten");
    map.insert(11, "eleven");
    map.insert(12, "twelve");
    map.insert(13, "thirteen");
    map.insert(14, "fourteen");
    map.insert(15, "fifteen");
    map.insert(16, "sixteen");
    map.insert(17, "seventeen");
    map.insert(18, "eighteen");
    map.insert(19, "nineteen");
    map.insert(20, "twenty");
    map.insert(30, "thirty");
    map.insert(40, "forth");
    map.insert(50, "fifty");
    map.insert(60, "sixty");
    map.insert(70, "seventy");
    map.insert(80, "eighty");
    map.insert(90, "ninety");
    let mut ret = String::new();
    let mut num = num;
    let tmp = num / 100;
    if tmp != 0 {
        ret.push_str(map.get(&tmp).unwrap());
        ret.push_str(" hundred ");
        if num % 100 != 0 {
            ret.push_str("and ");
        }
    }
    num %= 100;
    if num <= 20 {
        ret.push_str(map.get(&num).unwrap());
    } else {
        if num / 10 != 0 {
            ret.push_str(map.get(&(num / 10 * 10)).unwrap());
        }
        if num % 10 != 0 {
            ret.push_str(map.get(&(num % 10)).unwrap());
        }
    }
    ret
}

fn len(s: String) -> i32 {
    println!("{}", s);
    s.chars().filter(|&x| x.is_alphabetic()).count() as i32
}

fn main() {
    let mut ans = 0;
    for i in 1..1000 {
        ans += len(tran(i));
    }
    ans += len(String::from("one thousand"));
    println!("{}", ans);
}
