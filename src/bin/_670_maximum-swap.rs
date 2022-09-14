impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        use std::collections::HashMap;
        let mut dict: HashMap<u8, usize> = HashMap::new();
        let num_string = num.to_string();
        let mut num_bytes = num_string.as_bytes().to_vec();

        num_bytes.iter().enumerate()
            .for_each(|(i, &v)| { dict.insert(v, i); });

        let mut sorted_num_bytes = num_bytes.clone();
        sorted_num_bytes.sort_by(|a, b| b.cmp(a));

        for i in 0..num_bytes.len() {
            if sorted_num_bytes[i] > num_bytes[i] {
                let temp = num_bytes[i];
                num_bytes[i] = sorted_num_bytes[i];
                let &idx = dict.get(&num_bytes[i]).unwrap();
                num_bytes[idx] = temp;
                break;
            }
        }

        num_bytes.iter()
            .map(|&u| (u as char).to_string())
            .collect::<Vec<String>>()
            .join("").parse::<i32>().unwrap()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::maximum_swap(2736), 7236);
    assert_eq!(Solution::maximum_swap(9973), 9973);
    assert_eq!(Solution::maximum_swap(1993), 9913);
    assert_eq!(Solution::maximum_swap(98368), 98863);
}