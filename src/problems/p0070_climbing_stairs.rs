pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        (1..n).fold((1, 1), |(res, prev), _| (res + prev, res)).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}