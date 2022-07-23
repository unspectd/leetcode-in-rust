pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrack(buffer: &mut Vec<i32>, start: i32, end: i32, results: &mut Vec<Vec<i32>>) {
            let left = buffer.capacity() - buffer.len();
            if left == 0 {
                results.push(buffer.clone());
                return;
            }

            for i in start..=(end + 1 - left as i32) {
                buffer.push(i);
                backtrack(buffer, i + 1, end, results);
                buffer.pop();
            }
        }

        let mut results = Vec::new();
        let mut buffer = Vec::with_capacity(k as usize);
        backtrack(&mut buffer, 1, n, &mut results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (n, k, result)
        struct TestCase(i32, i32, Vec<Vec<i32>>);
        let test_cases = [
            TestCase(
                4,
                3,
                vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]],
            ),
            TestCase(
                4,
                2,
                vec![
                    vec![1, 2],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![3, 4],
                ],
            ),
            TestCase(1, 1, vec![vec![1]]),
            TestCase(2, 2, vec![vec![1, 2]]),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::combine(test_case.0, test_case.1), test_case.2);
        }
    }
}