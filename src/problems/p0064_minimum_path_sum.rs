pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut dp = vec![i32::MAX; m + 1];

        dp[1] = 0;
        for i in 0..n {
            for j in 0..m {
                let j_dp = j + 1; // padding correction
                dp[j_dp] = grid[i][j] + dp[j_dp].min(dp[j_dp - 1]);
            }
        }

        dp[m]
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
            TestCase(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]], 7),
            TestCase(vec![vec![1, 2, 3], vec![4, 5, 6]], 12),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::min_path_sum(test_case.0), test_case.1);
        }
    }
}