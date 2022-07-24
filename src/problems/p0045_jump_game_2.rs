pub struct Solution;

use std::cmp::max;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let (mut steps, mut cur_step_max, mut max_reachable) = (0, 0usize, 0usize);
        for i in 0..nums.len() - 1 {
            max_reachable = max(max_reachable, i + nums[i] as usize);
            if i == cur_step_max {
                steps += 1;
                cur_step_max = max_reachable;
            }
        }

        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }
}