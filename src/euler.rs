

#[cfg(test)]
mod euler {

  #[test]
    // E9. Find the only Pythagorean triplet, {a, b, c}, for which a + b + c = 1000
  fn triplet() {
    let target_sum = 1000; 
    let mut result = 0;
    for a in 1..target_sum {
        for b in a + 1..target_sum {
            let c: i32 = target_sum - a - b;
            if c * c == a * a + b * b  {
                result  = a * b * c;
                break;
            }
        }
    }
    assert_eq!(result, 31875000);
  }
}