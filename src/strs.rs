pub struct Solution;
impl Solution {

  fn backspace_compare(s1: &str, s2 : &str) -> bool {
      fn process_str(s: &str) -> String {
        let stack = &mut Vec::<char>::new();
        for ch in s.chars() {
          if ch != '#' {
              stack.push(ch);
          } else if !stack.is_empty() {
              stack.pop();
          }
        }
        return stack.iter().collect()
      }
    return process_str(s1) == process_str(s2)
  }


  fn len_of_longest_substr_norepeat(s: &str) -> i32 {
    if s.len() <= 1 {
      return s.len() as i32;
    };

    let (mut result, mut l, b, mut seen) = (0, 0, s.as_bytes(), [u16::MAX; 255]);
    for r in 0..b.len() {
      let c = b[r];
      let seen_idx = seen[c as usize];
      if seen_idx >= l && seen_idx != u16::MAX {
        l = seen_idx + 1;
      }
      seen[c as usize] = r as u16;
      result = std::cmp::max(result, (r as i32 - l as i32) + 1);
    }
    result
  }

  fn is_valid_palindrome_with_deletion_one(s: &str) -> bool {
    fn is_palindrome(b: &[u8], mut l: usize, mut r: usize) -> bool {
      while l < r {
        if b[l] != b[r] {
          return false;
        }
        l += 1;
        r -= 1;
      }
      true
    }
    
    let (mut l, mut r, b) = (0, s.len()-1, s.as_bytes());
      while l < r {
        if b[l] != b[r] {
          return is_palindrome(b, l + 1, r) || is_palindrome(b, l, r - 1);
        }
        l += 1;
        r -= 1;
      }
      true
    }

  pub fn longest_common_prefix(ls: &[&str]) -> String {
      for (i, &ch) in ls[0].as_bytes().iter().enumerate() {
      for &s in &ls[1..] {
          if s.as_bytes().get(i) != Some(&ch) {
          return ls[0][..i].to_string()
          }
      }
    }
    "".to_string()
  }
}

#[cfg(test)]
mod test {
  use crate::strs::Solution;

  #[test]
  fn longest_common_prefix_test() {
    let ts = vec![(vec!["flower", "flow", "flight"],  "fl" )];
    for (i, t) in ts.iter().enumerate() {
      assert_eq!( Solution::longest_common_prefix(&t.0), t.1, "Failed {}", i);
    }
  } 

  // https://leetcode.com/problems/valid-palindrome-ii/
  #[test]
  fn is_valid_palindrome_with_deletion_one_test() {
    let ts = vec![("abc", false),("abca", true), ("aba", true)];
    for (_, t) in ts.iter().enumerate() {
        assert_eq!(Solution::is_valid_palindrome_with_deletion_one(t.0) , t.1, "Failed {}", t.0) 
    }
  }

  // https://leetcode.com/problems/longest-substring-without-repeating-characters/    
  #[test]
  fn length_of_longest_substring_test() {
    let ts = vec![("dvdf",  3), ("abba", 2), ("bbb", 1), ("123", 3),
        ("au",  2), ("abcdabcdef", 6), ("pwwkew", 3), (" ",  1)]; 
    for (_, t) in ts.iter().enumerate() {
          assert_eq!(Solution::len_of_longest_substr_norepeat(&t.0), t.1, "Failed {}", t.0) 
    }
  }

  // https://leetcode.com/problems/backspace-string-compare/
  #[test]
  fn backspace_compare_test() {
    let ts = vec![("ab#c", "ad#c" , true), ("ab##", "c#d#" , true), ("a#c", "b" , false)];
    for (_, t) in ts.iter().enumerate() {
          assert_eq!(Solution::backspace_compare(t.0, t.1), t.2, "Failed {} {}", t.0, t.1) 
    }
  }

}
