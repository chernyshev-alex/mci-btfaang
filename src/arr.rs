use std::collections::HashMap;

pub struct Solution;
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
    let (mut best_area, mut x1, mut x2) = (0, 0, height.len() - 1);
    while x1 < x2 {
      let area = std::cmp::min(height[x1], height[x2]) * (x2 - x1) as i32;
      best_area = std::cmp::max(area, best_area);
      if height[x1] <= height[x2] {
        x1 += 1
      } else {
        x2 -= 1
      }
    }
    best_area
  }

  pub fn trapped_water_bf(height: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..height.len() {
      let (mut li, mut ri, mut lmax, mut rmax) = (i, i, 0, 0);
      loop {
        lmax = std::cmp::max(lmax, height[li]);
        li = if li > 0 { li - 1 } else { break };
      }
      while ri < height.len() {
        rmax = std::cmp::max(rmax, height[ri]);
        ri = ri + 1;
      }
      let current_water = std::cmp::min(lmax, rmax) - height[i];
      if current_water >=0 {
        result += current_water;
      }
    }
    result
  }

  pub fn trapped_water_opt(height: Vec<i32>) -> i32 {
    let mut result = 0;
    // TODO 
    result
  }
}

#[cfg(test)]
mod test {
  use crate::arr::Solution;

  #[test]
  fn two_sum_opt() {
    let result = Solution::two_sum([1, 3, 7, 9, 2].to_vec(), 11);
    assert_eq!(result, vec![3, 4]);
  }

  #[test]
  fn container_with_most_water() {
    let result = Solution::max_area([4, 8, 1, 2, 3, 9].to_vec());
    assert_eq!(result, 32);
  }

  #[test]
  fn trapped_water_brute_force() {
    let result = Solution::trapped_water_bf([0, 1, 0, 2, 1, 0, 3, 1, 0, 1, 2].to_vec());
    assert_eq!(result, 8);
  }

  #[test]
  fn trapped_water_opt() {
    let result = Solution::trapped_water_opt([0, 1, 0, 2, 1, 0, 3, 1, 0, 1, 2].to_vec());
    assert_eq!(result, 8);
  }
}
