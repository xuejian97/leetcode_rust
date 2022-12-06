/*
You are given a string word that consists of digits and lowercase English letters.

You will replace every non-digit character with a space. For example, "a123bc34d8ef34" will become " 123  34 8  34". Notice that you are left with some integers that are separated by at least one space: "123", "34", "8", and "34".

Return the number of different integers after performing the replacement operations on word.

Two integers are considered different if their decimal representations without any leading zeros are different.



Example 1:

Input: word = "a123bc34d8ef34"
Output: 3
Explanation: The three different integers are "123", "34", and "8". Notice that "34" is only counted once.
Example 2:

Input: word = "leet1234code234"
Output: 2
Example 3:

Input: word = "a1b01c001"
Output: 1
Explanation: The three integers "1", "01", and "001" all represent the same integer because
the leading zeros are ignored when comparing their decimal values.


Constraints:

1 <= word.length <= 1000
word consists of digits and lowercase English letters.
 */
use std::collections::HashSet;

impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        let mut set = HashSet::new();
        let mut current_number = "".to_owned();
        for &x in word.as_bytes() {
            if x < 48 || x > 57 {
                if current_number.len() > 0 {
                    set.insert(current_number);
                    current_number = "".to_owned();
                }
                continue;
            }

            if x == 48 && current_number == "0".to_owned() {
                continue;
            }

            if x > 48 && current_number == "0".to_owned() {
                current_number = "".to_owned();
            }

            current_number.push(x as char);
        }

        if current_number.len() > 0 {
            set.insert(current_number);
        }

        set.len() as i32
    }
}

struct Solution;

fn main() {
    print!("{}", '0' as u8);
    print!("{}", '9' as u8);
}