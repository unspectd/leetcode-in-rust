pub struct Solution;

const DIRECTIONS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let (mut n, mut m) = (board.len(), board[0].len());

        fn dfs(x: usize, y: usize, board: &mut Vec<Vec<char>>, target: &[char]) -> bool {
            if target.len() == 0 {
                return true;
            }

            let (mut n, mut m) = (board.len(), board[0].len());
            if x >= n || y >= m {
                return false;
            }

            let ch = board[x][y];
            board[x][y] = '.';
            if ch == target[0] {
                if DIRECTIONS.iter().any(|(dx, dy)| {
                    dfs(
                        x.wrapping_add(*dx),
                        y.wrapping_add(*dy),
                        board,
                        &target[1..],
                    )
                }) {
                    return true;
                }
            }
            board[x][y] = ch;

            false
        }

        let chars = word.chars().collect::<Vec<char>>();
        for x in 0..n {
            for y in 0..m {
                if dfs(x, y, &mut board, &chars) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        assert_eq!(Solution::exist(board.clone(), "ABCCED".to_string()), true);
        assert_eq!(Solution::exist(board.clone(), "SEE".to_string()), true);
        assert_eq!(Solution::exist(board.clone(), "ABCB".to_string()), false);
    }
}