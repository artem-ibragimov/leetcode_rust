fn max_profit(prices: Vec<i32>) -> i32 {
   if prices.is_empty() { return 0; }
   let mut prev_price: i32 = *prices.first().unwrap();
   let mut profit: i32 = 0;
   for price in prices {
      if price - prev_price > 0 { profit += price - prev_price; }
      prev_price = price;
   }
   profit
}

#[cfg(test)]
mod tests {
   #[test]
   fn max_profit_test() {
      assert_eq!(super::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
      // Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
      //              Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
      assert_eq!(super::max_profit(vec![1, 2, 3, 4, 5]), 4);
      // Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
      //              Note that you cannot buy on day 1, buy on day 2 and sell them later, as you are
      //              engaging multiple transactions at the same time. You must sell before buying again.
      assert_eq!(super::max_profit(vec![7, 6, 4, 3, 1]), 0);
      // Explanation: In this case, no transaction is done, i.e. max profit = 0.
   }
}
