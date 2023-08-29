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

   // https://leetcode.com/problems/longest-substring-without-repeating-characters/

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

  // https://leetcode.com/problems/valid-palindrome-ii/

  fn valid_palindrome(s: &str) -> bool {
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
    struct TS { input: Vec<&'static str>, expected: &'static str, }
    let ts = vec![ 
      TS { input : vec!["flower", "flow", "flight"], expected : "fl" },
    ];

    for (i, t) in ts.iter().enumerate() {
      assert_eq!( Solution::longest_common_prefix(&t.input), t.expected, "Failed {}", i);
    }
  } 

  #[test]
  fn valid_palindrome() {
    struct TS { input: &'static str, exp : bool}
    let ts = vec![
      TS{input: "abc", exp : false}, 
      TS{input : "abca", exp : true}, 
    ]; 
    for (i, t) in ts.iter().enumerate() {
        assert_eq!(Solution::valid_palindrome(t.input), t.exp, "Failed {}", i) 
    }
  }

  // https://leetcode.com/problems/longest-substring-without-repeating-characters/    
  #[test]
  fn length_of_longest_substring_test() {
    struct TS { input: &'static str, exp : i32}
    let ts = vec![
        TS{input: "dvdf", exp : 3}, 
        TS{input: "abba", exp : 2},
        TS{input: "bbb", exp : 1},
        TS{input: "123", exp : 3},
        TS{input: "au", exp : 2},
        TS{input: "abcdabcdef", exp : 6},
        TS{input: "pwwkew", exp : 3},
        TS{input: " ", exp : 1},
    ]; 

    for (_, t) in ts.iter().enumerate() {
          assert_eq!(Solution::len_of_longest_substr_norepeat(&t.input), 
            t.exp, "Failed {}", t.input) 
    }
  }

  // https://leetcode.com/problems/backspace-string-compare/
  #[test]
  fn backspace_compare_test() {
    struct TS { input1: &'static str, input2: &'static str, exp : bool}
    let ts = vec![
        TS{input1: "ab#c", input2 : "ad#c" , exp : true}, 
        TS{input1: "ab##", input2 : "c#d#" , exp : true}, 
        TS{input1: "a#c", input2 : "b" , exp : false}, 
    ];

    for (_, t) in ts.iter().enumerate() {
          assert_eq!(Solution::backspace_compare(t.input1, t.input2),
            t.exp, "Failed {} {}", t.input1, t.input2) 
    }
  }

}
