#[cfg(test)]
mod tests {
    fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let mut prev_price: i32 = *prices.first().unwrap();
        let mut profit: i32 = 0;
        for price in prices {
            if price - prev_price > 0 {
                profit += price - prev_price;
            }
            prev_price = price;
        }
        profit
    }
    #[test]
    fn max_profit_test() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
