// TODO

impl Solution {
    pub fn preimage_size_fzf(k: i32) -> i32 {
        Solution::help(k + 1) - Solution::help(k)
    }

    pub fn help(k: i32) -> i32 {
        let mut r = 5 * k;
        let mut l = 0;
        while l <= r {
            let mid = (l + r) / 2;
            if Solution::zeta(mid) < k {
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        r + 1
    }

    pub fn zeta(mut x: i32) -> i32 {
        let mut res = 0;
        while x != 0 {
            res += x / 5;
            x /= 5;
        }
        res
    }
}

struct Solution;

fn main() {
    // let i = Solution::preimage_size_fzf(0);
    // println!("{}", i);
    println!("{}", (5+0)/2);
}