pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid[0].len();
        let mut dp = vec![0; m];
        dp[0] = 1;

        for row in &grid {
            for j in 0..m {
                if row[j] == 1 {
                    dp[j] = 0;
                } else if j > 0 {
                    dp[j] += dp[j - 1];
                }
            }
        }

        dp[m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (grid, result)
        struct TestCase(Vec<Vec<i32>>, i32);
        let test_cases = [
            TestCase(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]], 2),
            TestCase(vec![vec![0, 1], vec![0, 0]], 1),
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::unique_paths_with_obstacles(test_case.0),
                test_case.1
            );
        }
    }
}