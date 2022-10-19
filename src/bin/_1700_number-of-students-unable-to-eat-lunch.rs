impl Solution {
    pub fn count_students(mut students: Vec<i32>, mut sandwiches: Vec<i32>) -> i32 {
        let mut ans = 0;

        while ans != students.len() {
            if students[0] == sandwiches[0] {
                students.remove(0);
                sandwiches.remove(0);
                ans = 0;
            } else {
                let s = students[0];
                students.remove(0);
                students.push(s);
                ans += 1;
            }
        }

        ans as i32
    }
}

struct Solution;

fn main(){}