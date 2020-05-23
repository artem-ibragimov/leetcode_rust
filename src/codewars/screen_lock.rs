// Screen Locking Patterns
// https://www.codewars.com/kata/585894545a8a07255e0002f1

#[cfg(test)]
mod tests {
   struct Graph {
      /** (neccessary node to became adjacent, Adjacent node) */
      adjacents: [[(Option<usize>, usize); 8]; 9],
   }
   impl Graph {
      fn new() -> Graph {
         Graph {
            adjacents: [
               [
                  // A
                  (None, 1),
                  (Some(1), 2),
                  (None, 3),
                  (None, 4),
                  (None, 5),
                  (Some(3), 6),
                  (None, 7),
                  (Some(4), 8),
               ],
               [
                  // B
                  (None, 0),
                  (None, 2),
                  (None, 3),
                  (None, 4),
                  (None, 5),
                  (None, 6),
                  (Some(4), 7),
                  (None, 8),
               ],
               // C
               [
                  (Some(1), 0),
                  (None, 1),
                  (None, 3),
                  (None, 4),
                  (None, 5),
                  (Some(4), 6),
                  (None, 7),
                  (Some(5), 8),
               ],
               // D
               [
                  (None, 0),
                  (None, 1),
                  (None, 2),
                  (None, 4),
                  (Some(4), 5),
                  (None, 6),
                  (None, 7),
                  (None, 8),
               ],
               // E
               [
                  (None, 0),
                  (None, 1),
                  (None, 2),
                  (None, 3),
                  (None, 5),
                  (None, 6),
                  (None, 7),
                  (None, 8),
               ],
               // F
               [
                  (None, 0),
                  (None, 1),
                  (None, 2),
                  (Some(4), 3),
                  (None, 4),
                  (None, 6),
                  (None, 7),
                  (None, 8),
               ],
               // G
               [
                  (Some(3), 0),
                  (None, 1),
                  (Some(4), 2),
                  (None, 3),
                  (None, 4),
                  (None, 5),
                  (None, 7),
                  (Some(7), 8),
               ],
               // H
               [
                  (None, 0),
                  (Some(4), 1),
                  (None, 2),
                  (None, 3),
                  (None, 4),
                  (None, 5),
                  (None, 6),
                  (None, 8),
               ],
               // I
               [
                  (Some(4), 0),
                  (None, 1),
                  (Some(5), 2),
                  (None, 3),
                  (None, 4),
                  (None, 5),
                  (Some(7), 6),
                  (None, 7),
               ],
            ],
         }
      }
      fn indx(c: char) -> usize {
         c as usize - 'A' as usize
      }
      fn count_paths(&self, node: usize, len: u8, visited: &mut Vec<usize>) -> u64 {
         if len < 2 { return len as u64; }
         visited.push(node);
         let mut paths: u64 = 0;
         for (condition, next_node) in self.adjacents.get(node).unwrap() {
            if visited.contains(next_node) { continue; }
            match condition {
               None => paths += self.count_paths(*next_node, len - 1, visited),
               Some(need_node) => {
                  if !visited.contains(need_node) { continue; }
                  paths += self.count_paths(*next_node, len - 1, visited)
               }
            }
         }
         visited.pop();
         paths
      }
   }
   fn count_patterns(from: char, length: u8) -> u64 {
      let graph = Graph::new();
      let mut visited: Vec<usize> = vec![];
      graph.count_paths(Graph::indx(from), length, &mut visited)
   }

   #[test]
   fn basic_tests() {
      assert_eq!(count_patterns('A', 0), 0);
      assert_eq!(count_patterns('A', 10), 0);
      assert_eq!(count_patterns('B', 1), 1);
      assert_eq!(count_patterns('C', 2), 5);
      assert_eq!(count_patterns('D', 3), 37);
      assert_eq!(count_patterns('E', 4), 256);
      assert_eq!(count_patterns('E', 8), 23280);
   }
}
