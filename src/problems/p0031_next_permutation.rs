pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut lo = nums.len() - 1;
        while lo > 0 && nums[lo - 1] >= nums[lo] {
            lo -= 1
        }

        if lo > 0 {
            let mut hi = lo;
            while hi < nums.len() - 1 && nums[hi + 1] > nums[lo - 1] {
                hi += 1
            }
            nums.swap(lo - 1, hi);
        }

        nums[lo..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // (nums, expected_nums_after_run)
        struct TestCase(Vec<i32>, Vec<i32>);
        let mut test_cases = [
            TestCase(vec![1, 2, 3], vec![1, 3, 2]),
            TestCase(vec![1, 3, 2], vec![2, 1, 3]),
            TestCase(vec![3, 1, 2], vec![3, 2, 1]),
            TestCase(vec![3, 2, 1], vec![1, 2, 3]),
            TestCase(vec![1, 5, 1], vec![5, 1, 1]),
            TestCase(vec![1, 1, 5], vec![1, 5, 1]),
            TestCase(vec![1, 2, 4, 5, 3], vec![1, 2, 5, 3, 4]),
        ];

        for test_case in &mut test_cases {
            Solution::next_permutation(&mut test_case.0);
            assert_eq!(test_case.0, test_case.1)
        }
    }
}