
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        for x in nums {
            if x > 0 {
                set.insert(x);
            }
        }
        set.len() as i32
    }
    // 排序 + 模拟
    pub fn minimum_operations_bak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut round = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 { continue; }
            round += 1;
            for j in (i + 1..nums.len()).rev() {
                nums[j] -= nums[i];
                if j == nums.len() && nums[j] <= 0 {
                    break;
                }
            }
        }

        round
    }
}

struct Solution;

fn main() {}