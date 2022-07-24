pub struct Solution;

impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let (mut left, mut right) = (0, heights.len() - 1);

        while left < right {
            let height = heights[left].min(heights[right]);
            max_area = max_area.max(height * (right - left) as i32);

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
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }
}