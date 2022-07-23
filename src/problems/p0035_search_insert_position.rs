pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0i32, nums.len() as i32 - 1);

        while lo <= hi {
            let mid = (lo + hi) / 2;

            match target.cmp(&nums[mid as usize]) {
                Ordering::Equal => {
                    return mid;
                }
                Ordering::Less => {
                    hi = mid - 1;
                }
                Ordering::Greater => lo = mid + 1,
            }
        }

        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}