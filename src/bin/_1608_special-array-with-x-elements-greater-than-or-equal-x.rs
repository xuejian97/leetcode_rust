impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));
        for i in 1..=nums.len() {
            if nums[i - 1] >= i as i32 && (i == nums.len() || nums[i] < i as i32) {
                return i as i32;
            }
        }
        -1
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::special_array(vec![0, 4, 3, 0, 4]),
        3
    );
}