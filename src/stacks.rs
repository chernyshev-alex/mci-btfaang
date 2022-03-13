use std::collections::HashMap;
use std::str;
struct Solution;
impl Solution {

    pub fn is_valid(s: String) -> bool {
        if s.len() == 0 {
            return  true;
        }

        let mut parens =std::collections::HashMap::<char, char>::new();
        parens.insert('(', ')');
        parens.insert('[', ']');
        parens.insert('{', '}');

        let mut stack: Vec<char> = vec![];
        for ch in s.chars() {
            if parens.contains_key(&ch) {
                stack.push(ch);
            } else {
                let left = stack.pop();
                if left.is_some() {
                    let valid = parens.get(&left.unwrap());
                    if ch != *valid.unwrap() {
                        return false
                    }
                } else { 
                    return false;
                }
            }   
        }
        stack.len() == 0
    }

    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = vec![];
        let mut mut_s = s.clone();
        let result = unsafe { mut_s.as_bytes_mut() };
        let mut s_mut = unsafe { s.char_indices() } ;
        for (i, ch) in s_mut {   
            match ch  {
                '(' => stack.push(i),
                ')' if !stack.is_empty() => { stack.pop(); }, 
                ')'  => { 
                    result[i] = ' ' as u8;
                    },          
               _  => (),
            }     
        }
        while let Some(i) = stack.pop() {
            result[i] = ' ' as u8;
        }
        let mut res = str::from_utf8(result).unwrap().to_string();
        res.retain(|c| !c.is_whitespace());
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // https://leetcode.com/problems/valid-parentheses/
    #[test]
    fn valid_parentheses() {
        assert_eq!(true, Solution::is_valid("{[()]}".to_string()));
        assert_eq!(false, Solution::is_valid("{(])}".to_string()));
    }

    // https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses/
    #[test]
    fn min_remove_to_make_valid_test() {
        assert_eq!("ab(cd)", Solution::min_remove_to_make_valid("(ab(cd)".to_string()));
        assert_eq!("lee(t(c)o)de", Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string()));
        assert_eq!("ab(c)d", Solution::min_remove_to_make_valid("a)b(c)d".to_string()));
        assert_eq!("", Solution::min_remove_to_make_valid("))((".to_string()));
    }

}