/*
Sometimes people repeat letters to represent extra feeling. For example:

"hello" -> "heeellooo"
"hi" -> "hiiii"
In these strings like "heeellooo", we have groups of adjacent letters that are all the same: "h", "eee", "ll", "ooo".

You are given a string s and an array of query strings words. A query word is stretchy if it can be made to be equal to s by any number of applications of the following extension operation: choose a group consisting of characters c, and add some number of characters c to the group so that the size of the group is three or more.

For example, starting with "hello", we could do an extension on the group "o" to get "hellooo", but we cannot get "helloo" since the group "oo" has a size less than three. Also, we could do another extension like "ll" -> "lllll" to get "helllllooo". If s = "helllllooo", then the query word "hello" would be stretchy because of these two extension operations: query = "hello" -> "hellooo" -> "helllllooo" = s.
Return the number of query strings that are stretchy.



Example 1:

Input: s = "heeellooo", words = ["hello", "hi", "helo"]
Output: 1
Explanation:
We can extend "e" and "o" in the word "hello" to get "heeellooo".
We can't extend "helo" to get "heeellooo" because the group "ll" is not size 3 or more.
Example 2:

Input: s = "zzzzzyyyyy", words = ["zzyy","zy","zyy"]
Output: 3


Constraints:

1 <= s.length, words.length <= 100
1 <= words[i].length <= 100
s and words[i] consist of lowercase letters.
 */
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let zip_s = Solution::zip_words(s);

        let mut ans = 0;
        for word in words {
            let zip_word = Solution::zip_words(word);
            if Solution::is_expressive(&zip_s, &zip_word) {
                ans +=1;
            }
        }

        ans
    }

    pub fn is_expressive(s: &Vec<(u8, i32)>, word: &Vec<(u8, i32)>) -> bool {
        if s.len() != word.len() { return false; }
        for i in 0..s.len() {
            let s_unit = s[i];
            let word_unit = word[i];
            if s_unit.0 != word_unit.0 { return false; }
            if s_unit.1 < word_unit.1 { return  false; }
            if s_unit.1 < 3 && s_unit.1 > word_unit.1 { return false }
        }
        true
    }

    pub fn zip_words(s: String) -> Vec<(u8, i32)> {
        let mut vec = vec![];
        let mut pre = 0;
        for &x in s.as_bytes() {
            if x != pre {
                pre = x;
                vec.push((x, 1));
            } else {
                let (_, cnt) = vec.pop().unwrap();
                vec.push((x, cnt + 1));
            }
        }
        vec
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::expressive_words("heeellooo".to_owned(), vec!["hello".to_owned(), "hi".to_owned(), "helo".to_owned()]), 1);
}