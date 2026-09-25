impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut cost = i32::MAX;
        for &price in prices.iter() {
            cost = cost.min(price);
            max_profit = max_profit.max(price - cost);
        }
        max_profit
    }
}
