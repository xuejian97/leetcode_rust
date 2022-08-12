/*
There are n peoplethat are split into some unknown number of groups. Each person is labeled with aunique IDfrom0ton - 1.

You are given an integer arraygroupSizes, where groupSizes[i]is the size of the group that personiis in. For example, ifgroupSizes[1] = 3, thenperson1must be in agroup of size3.

Returna list of groupssuch thateach personiis in a group of sizegroupSizes[i].

Each person shouldappear inexactly one group,and every person must be in a group. If there aremultiple answers, return any of them. It is guaranteed that there will be at least one valid solution for the given input.



Example 1:

Input: groupSizes = [3,3,3,3,3,1,3]
Output: [[5],[0,1,2],[3,4,6]]
Explanation: 
The first group is [5]. The size is 1, and groupSizes[5] = 1.
The second group is [0,1,2]. The size is 3, and groupSizes[0] = groupSizes[1] = groupSizes[2] = 3.
The third group is [3,4,6]. The size is 3, and groupSizes[3] = groupSizes[4] = groupSizes[6] = 3.
Other possible solutions are [[2,1,6],[5],[0,4,3]] and [[5],[0,6,2],[4,3,1]].
Example 2:

Input: groupSizes = [2,1,3,3,3,2]
Output: [[1],[0,5],[2,3,4]]


Constraints:

groupSizes.length == n
1 <= n<= 500
1 <=groupSizes[i] <= n


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/group-the-people-given-the-group-size-they-belong-to
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut map: HashMap<i32, Vec<Vec<i32>>> = HashMap::new();

        for (idx, &group_size) in group_sizes.iter().enumerate() {
            match map.get_mut(&group_size) {
                Some(gz_vec) => {
                    let i = gz_vec.len() - 1;
                    let x = gz_vec.get_mut(i).unwrap();
                    if x.len() == group_size as usize {
                        gz_vec.push(vec![idx as i32]);
                    } else {
                        x.push(idx as i32)
                    }
                }
                None => {
                    map.insert(group_size, vec![vec![idx as i32]]);
                }
            }
        }

        map.iter()
            .flat_map(|(_, v)| v.clone())
            .collect::<Vec<Vec<i32>>>()

    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3]),
        vec![
            vec![5],
            vec![0, 1, 2],
            vec![3, 4, 6],
        ]
    );

    assert_eq!(
        Solution::group_the_people(vec![2, 1, 3, 3, 3, 2]),
        vec![
            vec![1],
            vec![0, 5],
            vec![2, 3, 4],
        ]
    );
}