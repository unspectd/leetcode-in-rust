pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut result = Vec::new();

        let mut i = 0;
        while i < nums.len() - 2 && nums[i] <= 0 {
            let (mut lo, mut hi) = (i + 1, nums.len() - 1);

            while lo < hi {
                let sum = nums[i] + nums[lo] + nums[hi];
                match sum.cmp(&0) {
                    Ordering::Equal => {
                        result.push(vec![nums[i], nums[lo], nums[hi]]);

                        while lo < hi && nums[lo] == nums[lo + 1] {
                            lo += 1;
                        }
                        lo += 1;
                    }
                    Ordering::Less => {
                        lo += 1;
                    }
                    Ordering::Greater => {
                        hi -= 1;
                    }
                }
            }

            while i < nums.len() - 2 && nums[i] == nums[i + 1] {
                i += 1;
            }
            i += 1;
        }

        result
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
                vec![-1, 0, 1, 2, -1, -4],
                vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            ),
            TestCase(vec![0, 1, 1], vec![]),
            TestCase(vec![0, 0, 0], vec![vec![0, 0, 0]]),
        ];

        for test_case in test_cases {
            assert_eq!(Solution::three_sum(test_case.0), test_case.1);
        }
    }
}