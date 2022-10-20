impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let mut k = k - 1;
        let mut roadmap = vec![];
        for _ in 0..n {
            if k % 2 == 0 {
                roadmap.insert(0, 0);
            } else {
                roadmap.insert(0, 1);
            }
            k = k / 2;
        }
        let index_map = [[0, 1],[1,0]];
        let mut ans = 0;
        for x in roadmap {
            ans = index_map[ans][x];
        }

        ans as i32
    }
}

struct Solution;

fn main() {}