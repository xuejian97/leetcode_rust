
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut first_basket = (-1, 0, 0);
        let mut second_basket = (-1, 0, 0);
        for (idx, &fruit) in fruits.iter().enumerate() {
            if first_basket.0 == fruit {
                first_basket.1 += 1;
                first_basket.2 = idx;
                continue;
            } else if second_basket.0 == fruit {
                second_basket.1 += 1;
                second_basket.2 = idx;
                continue;
            }

            if first_basket.0 == -1 {
                first_basket.0 = fruit;
                first_basket.1 = 1;
                first_basket.2 = idx;
                continue;
            } else if second_basket.0 == -1 {
                second_basket.0 = fruit;
                second_basket.1 = 1;
                second_basket.2 = idx;
                continue;
            }

            if first_basket.0 != fruit && second_basket.0 != fruit {
                ans = ans.max(first_basket.1 + second_basket.1);
                if first_basket.2 > second_basket.2 {
                    first_basket.1 = first_basket.2 as i32 - second_basket.2 as i32;
                } else {
                    second_basket.1 = second_basket.2 as i32 - first_basket.2 as i32;
                    first_basket = second_basket;
                }
                second_basket = (fruit, 1, idx);
            }
        }
        ans.max(first_basket.1 + second_basket.1)
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::total_fruit(vec![1, 2, 1]), 3);
    assert_eq!(Solution::total_fruit(vec![0, 1, 2, 2]), 3);
    assert_eq!(Solution::total_fruit(vec![1, 2, 3, 2, 2]), 4);
    assert_eq!(
        Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]),
        5
    );
    assert_eq!(Solution::total_fruit(vec![1, 0, 1, 4, 1, 4, 1, 2, 3]), 5);
}
