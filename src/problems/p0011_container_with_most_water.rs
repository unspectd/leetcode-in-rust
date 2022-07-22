pub struct Solution;

use std::cmp::{max, min};

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = heights.len() - 1;

        while left < right {
            let height = min(heights[left], heights[right]);
            max_area = max(max_area, height * (right - left) as i32);

            while left < right && heights[left] <= height {
                left += 1
            }
            while left < right && heights[right] <= height {
                right -= 1
            }
        }

        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }
}