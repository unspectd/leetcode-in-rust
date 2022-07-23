pub struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(
            buffer: &mut Vec<i32>,
            candidates: &[i32],
            left: i32,
            results: &mut Vec<Vec<i32>>,
        ) {
            if left == 0 {
                results.push(buffer.clone());
                return;
            }

            for i in 0..candidates.len() {
                let cur = candidates[i];
                if cur > left {
                    return;
                }

                buffer.push(cur);
                backtrack(buffer, &candidates[i..], left - cur, results);
                buffer.pop();
            }
        }

        candidates.sort_unstable();

        let mut results = Vec::new();
        let mut buffer = Vec::new();
        backtrack(&mut buffer, &candidates, target, &mut results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (candidates, target, result)
        struct TestCase(Vec<i32>, i32, Vec<Vec<i32>>);
        let test_cases = [
            TestCase(vec![2, 3, 6, 7], 7, vec![vec![2, 2, 3], vec![7]]),
            TestCase(
                vec![2, 3, 5],
                8,
                vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            ),
            TestCase(vec![2], 1, vec![]),
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::combination_sum(test_case.0, test_case.1),
                test_case.2
            );
        }
    }
}