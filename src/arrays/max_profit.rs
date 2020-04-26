fn max_profit(prices: Vec<i32>) -> i32 {
   if prices.is_empty() {
      return 0;
   }
   let mut prev_price: i32 = *prices.first().unwrap();
   let dynamic: Vec<i32> = prices
      .into_iter()
      .map(|price| {
         let d = price - prev_price;
         prev_price = price;
         d
      })
      .collect();
   println!("dynamic {:?}", dynamic);
   let (price_to_buy, sum) = max_vec(dynamic);
   sum - price_to_buy
}

fn max_vec(data: Vec<i32>) -> (i32, i32) {
   use std::cmp::max;
   use std::iter::FromIterator;
   let len = data.len();
   if len <= 1 {
      return match len {
         0 => (0, 0),
         _ => (data[0], data[0]),
      };
   }
   println!("data {:?}", data);

   let mut mid_price: i32 = 0;
   let mut mid_sum: i32 = 0;
   for price in data[0..len / 2].iter().rev() {
      println!("compare sum {} & price {}", mid_sum, price);
      if *price == 0 {
         break;
      }
      if mid_sum + price < mid_sum {
         mid_price = *price;
         break;
      }
      mid_price = *price;
      mid_sum += price;
   }
   for price in data[len / 2..].iter() {
      println!("compare sum {} & price {}", mid_sum, price);
      if mid_sum + price <= mid_sum {
         break;
      }
      mid_sum += price;
   }

   let (left_price, left_sum) = max_vec(Vec::from_iter(data[..len / 2].iter().cloned()));
   print!("left {:?},  ", (left_price, left_sum));

   print!("mid {:?},  ", (mid_price, mid_sum));

   let (right_price, right_sum) = max_vec(Vec::from_iter(data[len / 2..].iter().cloned()));
   println!("right {:?}", (right_price, right_sum));
   let max_sum = max(mid_sum, max(left_sum, right_sum));
   if max_sum == left_sum {
      return (left_price, left_sum);
   }
   if max_sum == right_sum {
      return (right_price, right_sum);
   }
   (mid_price, mid_sum)
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
