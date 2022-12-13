/*
You are given an array arr which consists of only zeros and ones, divide the array into three non-empty parts such that all of these parts represent the same binary value.

If it is possible, return any [i, j] with i + 1 < j, such that:

arr[0], arr[1], ..., arr[i] is the first part,
arr[i + 1], arr[i + 2], ..., arr[j - 1] is the second part, and
arr[j], arr[j + 1], ..., arr[arr.length - 1] is the third part.
All three parts have equal binary values.
If it is not possible, return [-1, -1].

Note that the entire part is used when considering what binary value it represents. For example, [1,1,0] represents 6 in decimal, not 3. Also, leading zeros are allowed, so [0,1,1] and [1,1] represent the same value.



Example 1:

Input: arr = [1,0,1,0,1]
Output: [0,3]
Example 2:

Input: arr = [1,1,0,1,1]
Output: [-1,-1]
Example 3:

Input: arr = [1,1,0,0,1]
Output: [0,2]


Constraints:

3 <= arr.length <= 3 * 104
arr[i] is 0 or 1
 */
impl Solution {
    pub fn three_equal_parts(arr: Vec<i32>) -> Vec<i32> {
        let cnt_1 = arr.iter().filter(|&i| i == &1).count();
        if cnt_1 == 0 {
            return vec![0, arr.len() as i32 - 1];
        }
        if cnt_1 % 3 != 0 {
            return vec![-1, -1];
        }

        let mut tail_0_cnt = 0;
        for i in (0..arr.len()).rev() {
            if arr[i] == 1 {
                break;
            }
            tail_0_cnt += 1;
        }

        let part_1_cnt = cnt_1 / 3;

        let mut part_1 = vec![];
        let mut part_2 = vec![];
        let mut part_3 = vec![];

        let mut part_1_1_cnt = 0;
        let mut part_2_1_cnt = 0;
        let mut part_3_1_cnt = 0;

        let mut part_1_tail_0_cnt = 0;
        let mut part_2_tail_0_cnt = 0;
        let mut part_3_tail_0_cnt = 0;


        let mut ans = vec![];

        for (i, &x) in arr.iter().enumerate() {
            if part_1_1_cnt < part_1_cnt || part_1_tail_0_cnt < tail_0_cnt {
                if part_1_1_cnt == 0 && x == 0 {
                    continue;
                }
                if part_1_1_cnt == part_1_cnt {
                    if x != 0 { return vec![-1, -1]; }
                    part_1_tail_0_cnt += 1;
                }
                part_1.push(x);
                if x == 1 {
                    part_1_1_cnt += 1;
                }

                if part_1_1_cnt == part_1_cnt && part_1_tail_0_cnt == tail_0_cnt {
                    ans.push(i as i32);
                }
            } else if part_2_1_cnt < part_1_cnt || part_2_tail_0_cnt < tail_0_cnt {
                if part_2_1_cnt == 0 && x == 0 {
                    continue;
                }
                if part_2_1_cnt == part_1_cnt {
                    if x != 0 { return vec![-1, -1]; }
                    part_2_tail_0_cnt += 1;
                }
                part_2.push(x);
                if x == 1 {
                    part_2_1_cnt += 1;
                }
                if part_2_1_cnt == part_1_cnt && part_2_tail_0_cnt == tail_0_cnt {
                    ans.push(i as i32 + 1);
                }
            } else if part_3_1_cnt < part_1_cnt || part_3_tail_0_cnt < tail_0_cnt {
                if part_3_1_cnt == 0 && x == 0 {
                    continue;
                }
                if part_3_1_cnt == part_1_cnt {
                    if x != 0 { return vec![-1, -1]; }
                    part_3_tail_0_cnt += 1;
                }
                part_3.push(x);
                if x == 1 {
                    part_3_1_cnt += 1;
                }
            } else {
                return vec![-1, -1];
            }
        }

        for i in 0..part_1_cnt {
            if part_1[i] != part_2[i] || part_2[i] != part_3[i] {
                return vec![-1, -1];
            }
        }

        ans
    }
}

struct Solution;

fn main() {
    // assert_eq!(Solution::three_equal_parts(vec![1, 0, 1, 0, 1]), vec![0, 3]);
    // assert_eq!(Solution::three_equal_parts(vec![0, 1, 0, 1, 1, 0, 1, 1, 0, 1]), vec![3, 7]);
    assert_eq!(Solution::three_equal_parts(vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0])
               , vec![15, 32]);
}