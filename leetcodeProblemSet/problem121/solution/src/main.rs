struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut max = prices[0];
        let mut profit = 0;

        for i in 1..prices.len() {
            if prices[i] <= min {
                min = prices[i];
                max = prices[i];
                continue;
            }
            if prices[i] >= max {
                max = prices[i];
                if (max - min) > profit {
                    profit = max - min;
                }
            }
        }
        profit
    }
}
fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let prices_1 = vec![7, 6, 5, 4, 3, 2, 1];
    let prices_2 = vec![2, 4, 1];
    let prices_3 = vec![2, 4, 1, 6];
    println!("{}", Solution::max_profit(prices));
    println!("{}", Solution::max_profit(prices_1));
    println!("{}", Solution::max_profit(prices_2));
    println!("{}", Solution::max_profit(prices_3));
}
