pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut seen_at = HashMap::with_capacity(nums.len());
        nums.into_iter()
            .enumerate()
            .any(|(idx, num)| match seen_at.insert(num, idx) {
                Some(prev_idx) => idx - prev_idx <= k as usize,
                None => false,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (nums, k, result)
        struct TestCase(Vec<i32>, i32, bool);
        let test_cases = [
            TestCase(vec![1, 2, 3, 1], 3, true),
            TestCase(vec![1, 0, 1, 1], 1, true),
            TestCase(vec![1, 2, 3, 1, 2, 3], 2, false),
        ];

        for test_case in test_cases {
            assert_eq!(
                Solution::contains_nearby_duplicate(test_case.0, test_case.1),
                test_case.2
            );
        }
    }
}