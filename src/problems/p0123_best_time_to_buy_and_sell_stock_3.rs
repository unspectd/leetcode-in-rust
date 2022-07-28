pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut dp = vec![0; prices.len()];
        let (mut min_price, mut max_profit) = (prices[0], 0);
        for i in 1..prices.len() {
            min_price = min_price.min(prices[i]);
            max_profit = max_profit.max(prices[i] - min_price);
            dp[i] = max_profit;
        }

        max_profit = 0;
        let (mut max_price, mut total) = (prices[prices.len() - 1], 0);
        for i in (0..prices.len() - 1).rev() {
            max_price = max_price.max(prices[i]);
            max_profit = max_profit.max(max_price - prices[i]);
            total = total.max(max_profit + dp[i]);
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}