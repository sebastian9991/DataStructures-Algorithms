struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .windows(2)
            .fold(0, |acc, w| acc + (w[1] - w[0]).max(0))
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    let prices_2 = vec![1, 2, 3, 4, 5];
    let prices_3 = vec![7, 6, 4, 3, 1];
    println!("{}", Solution::max_profit(prices));
    println!("{}", Solution::max_profit(prices_2));
    println!("{}", Solution::max_profit(prices_3));
}
