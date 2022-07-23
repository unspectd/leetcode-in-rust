pub struct Solution;

impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
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

            let mut i = 0;
            while i < candidates.len() {
                let cur = candidates[i];
                if cur > left {
                    return;
                }

                buffer.push(cur);
                backtrack(buffer, &candidates[i + 1..], left - cur, results);
                buffer.pop();

                while i < candidates.len() - 1 && candidates[i + 1] == cur {
                    i += 1;
                }

                i += 1;
            }
        }

        candidates.sort();

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
            TestCase(
                vec![10, 1, 2, 7, 6, 1, 5],
                8,
                vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]],
            ),
            TestCase(vec![2, 5, 2, 1, 2], 5, vec![vec![1, 2, 2], vec![5]]),
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::combination_sum2(test_case.0, test_case.1),
                test_case.2
            );
        }
    }
}