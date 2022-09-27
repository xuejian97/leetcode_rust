impl Solution {
    pub fn missing_two(nums: Vec<i32>) -> Vec<i32> {
        let missed_two_sum = ((1 + nums.len() + 2) * (nums.len() + 2) / 2) as i32 - nums.iter().sum::<i32>();
        let mid = missed_two_sum / 2;
        let mut mid_sum = (1 + mid) * mid / 2;
        for x in nums {
            if x <= mid { mid_sum -= x; }
        }
        vec![mid_sum, missed_two_sum - mid_sum]
    }
}

struct Solution;

fn main() {
    let vec1 = Solution::missing_two(vec![1, 2, 3, 4, 6, 7, 9, 10]);
    println!("{:?}", vec1);
}