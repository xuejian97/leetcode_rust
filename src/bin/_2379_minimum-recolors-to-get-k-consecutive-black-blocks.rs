impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let white_u8 = 87_u8;
        let mut white_cnt = 0;
        let bytes = blocks.as_bytes();
        for i in 0..k as usize {
            if bytes[i] == white_u8 { white_cnt += 1; }
        }

        let mut min = white_cnt;

        for i in 1..bytes.len() - k as usize + 1 {
            if bytes[i - 1] == white_u8 { white_cnt -= 1; }
            if bytes[i + k as usize - 1] == white_u8 { white_cnt += 1; }
            if white_cnt < min {
                min = white_cnt;
            }
        }

        min
    }
}

struct Solution;

fn main() {
}