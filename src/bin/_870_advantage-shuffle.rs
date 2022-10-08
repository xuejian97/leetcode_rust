/*
You are given two integer arrays nums1 and nums2 both of the same length. The advantage of nums1 with respect to nums2 is the number of indices i for which nums1[i] > nums2[i].

Return any permutation of nums1 that maximizes its advantage with respect to nums2.



Example 1:

Input: nums1 = [2,7,11,15], nums2 = [1,10,4,11]
Output: [2,11,7,15]
Example 2:

Input: nums1 = [12,24,8,32], nums2 = [13,25,32,11]
Output: [24,32,8,12]


Constraints:

1 <= nums1.length <= 105
nums2.length == nums1.length
0 <= nums1[i], nums2[i] <= 109

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/advantage-shuffle
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n = nums1.len();
        let mut idx1 = vec![];
        let mut idx2 = vec![];
        for i in 0..n {
            idx1.push(i);
            idx2.push(i);
        }
        idx1.sort_by(|&a, &b| nums1[a].cmp(&nums1[b]));
        idx2.sort_by(|&a, &b| nums2[a].cmp(&nums2[b]));

        let mut ans = vec![0; n];
        let (mut left, mut right) = (0, n - 1);
        for i in 0..n {
            if nums1[idx1[i]] > nums2[idx2[left]] {
                ans[idx2[left]] = nums1[idx1[i]];
                left += 1;
            } else {
                ans[idx2[right]] = nums1[idx1[i]];
                if right > 0 {
                    right -= 1;
                }
            }
        }
        ans
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::advantage_count(
            vec![5621, 1743, 5532, 3549, 9581],
            vec![913, 9787, 4121, 5039, 1481],
        ),
        vec![1743, 9581, 5532, 5621, 3549]
    );
}