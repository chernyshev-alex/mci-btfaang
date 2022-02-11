use std::collections::HashMap;

struct Solution;
impl Solution {
  // given array of ints, return indices of the two numbers that add up to a given target
  // https://leetcode.com/problems/two-sum/
  pub fn two_sum(v: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for i1 in 0..v.len() {
      if let Some(i2) = m.get(&v[i1]) {
        return vec![*i2, i1 as i32];
      } else {
        m.insert(target - v[i1], i1 as i32);
      }
    }
    vec![]
  }

  // https://leetcode.com/problems/container-with-most-water/
  pub fn max_area(height: Vec<i32>) -> i32 { 
    let (mut best_area, mut x1, mut x2) = (0, 0, height.len()-1);
    while x1 < x2 {
      let area = std::cmp::min(height[x1], height[x2]) * (x2 - x1) as i32;
      best_area = if area > best_area { area } else { best_area };
      if height[x1] <= height[x2] { x1 += 1 } else { x2 -= 1 }
    }
    best_area
  }
}

#[cfg(test)]
mod test {
  use crate::arr::Solution;

  #[test]
  fn container_with_most_water() {
    let result = Solution::max_area([4,8,1,2,3,9].to_vec());
    assert_eq!(result, 32);
  }

  #[test]
  fn two_sum_opt() {
    let result = Solution::two_sum([1, 3, 7, 9, 2].to_vec(), 11);
    assert_eq!(result, vec![3, 4]);
  }
}
