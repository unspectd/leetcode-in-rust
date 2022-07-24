pub struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[1] = 0;

        for (i, row) in triangle.iter().enumerate() {
            for j in (0..=i).rev() {
                let j_dp = j + 1; // padding correction
                dp[j_dp] = dp[j_dp].min(dp[j_dp - 1]) + row[j]
            }
        }

        *dp.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (triangle, result)
        struct TestCase(Vec<Vec<i32>>, i32);
        let test_cases = [
            TestCase(
                vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]],
                11,
            ),
            TestCase(vec![vec![-10]], -10),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::minimum_total(test_case.0), test_case.1);
        }
    }
}