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

  pub fn length_of_longest_substring_bf(s: String) -> i32 {
    if s.len() <= 1 {
      return s.len() as i32;
    };
    let mut result = 0;
    let b = s.as_bytes();
    for i in 0..b.len() {
      let (mut partial_len, mut seen_chars) = (0, [0; 255]);

      for j in i..b.len() {
        let c = b[j] as usize;
        if seen_chars[c] == 0 {
          seen_chars[c] = c;
          partial_len += 1;
          result = std::cmp::max(result, partial_len);
        } else {
          break;
        }
      }
    }
    result
  }
}

#[cfg(test)]
mod test {
  use crate::strs::Solution;

  #[test]
  fn length_of_longest_substring_case1() {
    let result = Solution::length_of_longest_substring_bf("abcdabcdef".to_string());
    assert_eq!(result, 2);
  }

  #[test]
  fn length_of_longest_substring_space() {
    let result = Solution::length_of_longest_substring_bf(" ".to_string());
    assert_eq!(result, 1);
  }

  #[test]
  fn length_of_longest_substring() {
    let result = Solution::length_of_longest_substring_bf("pwwkew".to_string());
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
