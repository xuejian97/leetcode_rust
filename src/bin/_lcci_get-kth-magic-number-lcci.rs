impl Solution {
    pub fn get_kth_magic_number(k: i32) -> i32 {
        let mut dp = vec![0; k as usize + 1];
        dp[1] = 1;
        let mut ps = [1, 1, 1];
        for i in 2..=k as usize {
            let nums = [dp[ps[0]] * 3, dp[ps[1]] * 5, dp[ps[2]] * 7];
            dp[i] = *nums.iter().min().unwrap();
            for (idx, &v) in nums.iter().enumerate() {
                if dp[i] == v {
                    ps[idx] += 1;
                }
            }
        }
        dp[k as usize]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::get_kth_magic_number(1), 1);
    assert_eq!(Solution::get_kth_magic_number(5), 9);
}