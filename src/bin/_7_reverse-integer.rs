impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let i = if x > 0 { 1 } else { -1 };
        let mut res = 0i64;
        let mut x = x.abs() as i64;
        while x > 0 {
            res = res * 10 + x % 10;
            x /= 10;
        }

        let res = res * i;
        if res < i32::MIN as i64 || res > i32::MAX as i64 {
            0
        } else {
            res as i32
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(120));
    println!("{}", Solution::reverse(0));
    println!("{}", Solution::reverse(1534236469));
}