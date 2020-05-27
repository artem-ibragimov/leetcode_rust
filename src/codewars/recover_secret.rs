// https://www.codewars.com/kata/53f40dff5f9d31b813000774/train/rust
// There is a secret string which is unknown to you. 
// Given a collection of random triplets from the string, recover the original string.

// A triplet here is defined as a sequence of three letters such that each letter occurs
//  somewhere before the next in the given string. "whi" is a triplet for the string "whatisup".

// As a simplification, you may assume that no letter occurs more than once in the secret string.

// You can assume nothing about the triplets given to you other than that they are valid triplets 
// and that they contain sufficient information to deduce the original string. 
// In particular, this means that the secret string will never contain letters 
// that do not occur in one of the triplets given to you.
#[cfg(test)]
mod test {
   fn recover_secret(triplets: Vec<[char; 3]>) -> String {
      let mut couples: Vec<[char; 2]> = vec![];
      for trpl in triplets {
         let [a, b, c] = trpl;
         if !couples.contains(&[a, b]) { couples.push([a, b]); }
         if !couples.contains(&[b, c]) { couples.push([b, c]); }
      }
      let mut result: Vec<char> = vec![];
      while let Some(nxt) = next(&couples, &result) {
         result.push(nxt);
      }
      result.iter().collect()
   }
   fn next(couples: &Vec<[char; 2]>, exclude: &Vec<char>) -> Option<char> {
      for [a, _] in couples {
         if exclude.contains(a){ continue; }
         if couples.iter().find(|[ex_a, b]| a == b && !exclude.contains(ex_a)).is_none() {
            return Some(*a);
         };
      }
      for [_, b] in couples {
         if exclude.contains(b){ continue; }
         return Some(*b);
      }
      None
   }
   #[test]
   fn example_test() {
      assert_eq!(
      recover_secret(vec![
         ['t', 'u', 'p'],
         ['t', 's', 'u'],
         ['t', 'i', 's'],
         ['w', 'h', 'i'],
         ['w', 'h', 's'],
         ['h', 'a', 'p'],
         ['a', 't', 's'],
      ]),
         "whatisup"
      );
   }
}
