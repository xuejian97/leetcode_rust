impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();
        let (mut row_mask, mut col_mask) = (0, 0);
        for i in 0..n {
            row_mask |= board[0][i] << i;
            col_mask |= board[i][0] << i;
        }

        let reverse_row_mask = ((1 << n) - 1) ^ row_mask;
        let reverse_col_mask = ((1 << n) - 1) ^ col_mask;
        let (mut row_cnt, mut col_cnt) = (0, 0);
        for i in 0..n {
            let mut curr_row_mask = 0;
            let mut curr_col_mask = 0;
            for j in 0..n {
                curr_row_mask |= board[i][j] << j;
                curr_col_mask |= board[j][i] << j;
            }
            if curr_row_mask != row_mask && curr_row_mask != reverse_row_mask {
                return -1;
            } else if curr_row_mask == row_mask {
                row_cnt += 1;
            }
            if curr_col_mask != col_mask && curr_col_mask != reverse_col_mask {
                return -1;
            } else if curr_col_mask == col_mask {
                col_cnt += 1;
            }
        }

        let row_moves = Solution::get_moves(row_mask, row_cnt, n as i32);
        let col_moves = Solution::get_moves(col_mask, col_cnt, n as i32);
        if row_moves == -1 || col_moves == -1 { -1 } else { row_moves + col_moves }
    }

    pub fn get_moves(mask: i32, count: i32, n: i32) -> i32 {
        let ones = mask.count_ones();
        return if (n & 1) == 1 {
            if (n - 2 * ones as i32).abs() != 1 || (n - 2 * count).abs() != 1 {
                return -1;
            }
            if ones as i32 == (n >> 1) {
                (n as u32 / 2 - (mask as u32 & 0xAAAAAAAA).count_ones()) as i32
            } else {
                ((n as u32 + 1) / 2 - (mask as u32 & 0x55555555).count_ones()) as i32
            }
        } else {
            if ones as i32 != (n >> 1) || count != (n >> 1) {
                return -1;
            }
            let count_0 = n as u32 / 2 - (mask as u32 & 0xAAAAAAAA).count_ones();
            let count_1 = n as u32 / 2 - (mask as u32 & 0x55555555).count_ones();
            count_0.min(count_1) as i32
        };
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::moves_to_chessboard(
        vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1],
        ]
    ), 2);
}