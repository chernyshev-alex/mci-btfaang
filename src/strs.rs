pub struct Solution;
impl Solution {
  // https://leetcode.com/problems/backspace-string-compare/

  pub fn backspace_compare(s: String, t: String) -> bool {
    fn maybe_advance_iterator(
      it: &mut dyn Iterator<Item = char>,
      el: Option<char>,
    ) -> Option<char> {
      if el.is_some() && el.unwrap() == '#' {
        let mut skip_cnt = 2;
        let mut curr_element: Option<char> = None;
        while skip_cnt > 0 {
          skip_cnt -= 1;
          curr_element = it.next();
          if let Some(c) = curr_element {
            if c == '#' {
              skip_cnt += 2
            }
          }
        }
        return curr_element;
      }
      return el;
    }

    let (mut sv, mut tv) = (s.chars().rev(), t.chars().rev());
    loop {
      let (mut sv_elem, mut tv_elem) = (sv.next(), tv.next());
      if sv_elem.is_none() && tv_elem.is_none() {
        break;
      }

      sv_elem = maybe_advance_iterator(&mut sv, sv_elem);
      tv_elem = maybe_advance_iterator(&mut tv, tv_elem);

      if sv_elem.ne(&tv_elem) {
        return false;
      }
    }
    true
  }

  // https://leetcode.com/problems/longest-substring-without-repeating-characters/

  pub fn length_of_longest_substring_opt(s: String) -> i32 {
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

  pub fn valid_palindrome(s: String) -> bool {
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
  }

#[cfg(test)]
mod test {
  use crate::strs::Solution;

  #[test]
  fn valid_palindrome_case1() {
    let result = Solution::valid_palindrome("abc".to_string());
    assert_eq!(result, false);
  }

  #[test]
  fn valid_palindrome_case2() {
    let result = Solution::valid_palindrome("abca".to_string());
    assert_eq!(result, true);
  }

  #[test]
  fn length_of_longest_substring_case7() {
    let result = Solution::length_of_longest_substring_opt("dvdf".to_string());
    assert_eq!(result, 3);
  }

  #[test]
  fn length_of_longest_substring_case6() {
    let result = Solution::length_of_longest_substring_opt("abba".to_string());
    assert_eq!(result, 2);
  }

  #[test]
  fn length_of_longest_substring_case5() {
    let result = Solution::length_of_longest_substring_opt("bbb".to_string());
    assert_eq!(result, 1);
  }
  #[test]
  fn length_of_longest_substring_case4() {
    let result = Solution::length_of_longest_substring_opt("123".to_string());
    assert_eq!(result, 3);
  }

  #[test]
  fn length_of_longest_substring_case4_1() {
    let result = Solution::length_of_longest_substring_opt("au".to_string());
    assert_eq!(result, 2);
  }

  #[test]
  fn length_of_longest_substring_case3() {
    let result = Solution::length_of_longest_substring_opt("abcabcbb".to_string());
    assert_eq!(result, 3);
  }

  #[test]
  fn length_of_longest_substring_case2() {
    let result = Solution::length_of_longest_substring_opt("abcdabcdef".to_string());
    assert_eq!(result, 6);
  }

  #[test]
  fn length_of_longest_substring_case1() {
    let result = Solution::length_of_longest_substring_opt(" ".to_string());
    assert_eq!(result, 1);
  }

  #[test]
  fn length_of_longest_substring_case0() {
    let result = Solution::length_of_longest_substring_opt("pwwkew".to_string());
    assert_eq!(result, 3);
  }

  #[test]
  fn compare_test_let0() {
    let result = Solution::backspace_compare("nzp#o#g".to_string(), "b#nzp#o#g".to_string());
    assert_eq!(result, true);
  }

  #[test]
  fn compare_test_let() {
    let result = Solution::backspace_compare("bbbextm".to_string(), "bbb#extm".to_string());
    assert_eq!(result, false);
  }
}
