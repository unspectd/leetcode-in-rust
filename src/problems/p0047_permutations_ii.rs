pub struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(
            buffer: &mut Vec<i32>,
            candidates: &mut Vec<i32>,
            results: &mut Vec<Vec<i32>>,
        ) {
            if candidates.len() == 0 {
                results.push(buffer.clone());
                return;
            }

            for i in 0..candidates.len() {
                if i > 0 && candidates[i] == candidates[i - 1] {
                    continue;
                }

                let cur = candidates[i];
                candidates.remove(i);
                buffer.push(cur);
                backtrack(buffer, candidates, results);
                buffer.pop();
                candidates.insert(i, cur);
            }
        }

        nums.sort_unstable();
        let mut results = Vec::new();
        let mut buffer = Vec::with_capacity(nums.len());
        backtrack(&mut buffer, &mut nums.clone(), &mut results);

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (nums, result)
        struct TestCase(Vec<i32>, Vec<Vec<i32>>);
        let test_cases = [
            TestCase(
                vec![1, 1, 2],
                vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
            ),
            TestCase(
                vec![1, 2, 1],
                vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
            ),
            TestCase(
                vec![1, 2, 3],
                vec![
                    vec![1, 2, 3],
                    vec![1, 3, 2],
                    vec![2, 1, 3],
                    vec![2, 3, 1],
                    vec![3, 1, 2],
                    vec![3, 2, 1],
                ],
            ),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::permute_unique(test_case.0), test_case.1);
        }
    }
}