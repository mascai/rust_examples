/*
https://leetcode.com/problems/best-time-to-buy-and-sell-stock/?envType=study-plan-v2&envId=top-interview-150


*/


impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_sell = prices[0];
        let mut max_profit = 0;
        for i in 1..prices.len() {
            max_profit = std::cmp::max(max_profit, prices[i] - min_sell);
            min_sell = std::cmp::min(min_sell, prices[i]);
        }
        return max_profit;
    }
}
