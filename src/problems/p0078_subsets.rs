pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let cap = 2i32.pow(nums.len() as u32) as usize;
        let mut results = Vec::with_capacity(cap);
        results.push(Vec::new());

        for n in nums {
            for mut v in results.clone() {
                v.push(n);
                results.push(v);
            }
        }

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
                vec![1, 2, 3],
                vec![
                    vec![],
                    vec![1],
                    vec![2],
                    vec![1, 2],
                    vec![3],
                    vec![1, 3],
                    vec![2, 3],
                    vec![1, 2, 3],
                ],
            ),
            TestCase(vec![0], vec![vec![], vec![0]]),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::subsets(test_case.0), test_case.1);
        }
    }
}