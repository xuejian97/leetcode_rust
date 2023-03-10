impl Solution {
    pub fn compress_string(s: String) -> String {
        if s.len() == 0 { return s; }
        let bytes = s.as_bytes();
        let mut cur_byte = bytes[0];
        let mut cnt = 1;
        let mut res = String::from("");

        for i in 1..=bytes.len() {
            if i < bytes.len() && bytes[i] == cur_byte {
                cnt += 1;
            } else {
                res.push_str(&format!("{}{}", cur_byte as char, cnt));

                if i < bytes.len() {
                    cur_byte = bytes[i];
                    cnt = 1;
                }
            }
        }

        if res.len() >= s.len() { s } else { res }
    }
}

struct Solution;

fn main() {}