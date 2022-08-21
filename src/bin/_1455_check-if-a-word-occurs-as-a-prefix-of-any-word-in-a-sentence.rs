impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        let mut idx = -1;
        for (index, str) in sentence.split(" ").enumerate() {
            if str.starts_with(&search_word) {
                idx = index as i32 + 1;
                break;
            }
        }

        idx
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_prefix_of_word("i love eating burger".to_owned(), "burg".to_owned()), 4);
    assert_eq!(Solution::is_prefix_of_word("this problem is an easy problem".to_owned(), "pro".to_owned()), 2);
    assert_eq!(Solution::is_prefix_of_word("i am tired".to_owned(), "you".to_owned()), -1);
}