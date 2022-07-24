pub struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        fn backtrack(buffer: &mut Vec<i32>, start: i32, sum: i32, results: &mut Vec<Vec<i32>>) {
            let left = buffer.capacity() - buffer.len();
            if left == 0 {
                if sum == 0 {
                    results.push(buffer.clone());
                }
                return;
            }

            for n in start..=9 {
                if n * left as i32 > sum {
                    return;
                }

                buffer.push(n);
                backtrack(buffer, n + 1, sum - n, results);
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
        // (k, n, result)
        struct TestCase(i32, i32, Vec<Vec<i32>>);
        let test_cases = [
            TestCase(2, 6, vec![vec![1, 5], vec![2, 4]]),
            TestCase(3, 7, vec![vec![1, 2, 4]]),
            TestCase(3, 9, vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]),
            TestCase(4, 1, vec![]),
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::combination_sum3(test_case.0, test_case.1),
                test_case.2
            );
        }
    }
}