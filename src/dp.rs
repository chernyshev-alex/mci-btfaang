// O(N*N)
fn min_cost_climbing_stairs_top_bottom(cost: Vec<i32>) -> i32 {
    fn min_cost(n: i32, cost: &Vec<i32>) -> i32 {
        if n < 0 {
            return 0;
        } else if n == 0 || n == 1 {
            return cost[n as usize];
        }
        cost[n as usize] + std::cmp::min(min_cost(n - 1, cost), min_cost(n - 2, cost))
    }
    let ln = cost.len() as i32;
    return std::cmp::min(min_cost(ln - 1, &cost), min_cost(ln - 2, &cost));
}

// memoization
fn min_cost_climbing_stairs_top_bottom_memo(cost: Vec<i32>) -> i32 {
    if cost.len() == 0 {
        return 0;
    }
    if cost.len() == 1 {
        return cost[0];
    }
    let mut dp = vec![0; cost.len()];
    for i in 0..cost.len() {
        if i < 2 {
            dp[i] = cost[i]
        } else {
            dp[i] = cost[i] + std::cmp::min(dp[i - 1], dp[i - 2])
        }
    }
    std::cmp::min(dp[cost.len() - 1], dp[cost.len() - 2])
}

// memoization opt
fn min_cost_climbing_stairs_top_bottom_memo_opt(cost: Vec<i32>) -> i32 {
    if cost.len() == 0 {
        return 0;
    }
    if cost.len() == 1 {
        return cost[0];
    }
    let (mut dp1, mut dp2) = (cost[0], cost[1]);
    for i in 2..cost.len() {
        let cc = cost[i] + std::cmp::min(dp1, dp2);
        dp1 = dp2;
        dp2 = cc;
    }
    std::cmp::min(dp1, dp2)
}

fn knight_directions() -> [[i32; 2]; 8] {
    const DIRECTIONS: [[i32; 2]; 8] = [
        [-2, -1],
        [-2, 1],
        [-1, 2],
        [1, 2],
        [2, 1],
        [2, -1],
        [1, -2],
        [-1, -2],
    ];
    DIRECTIONS
}

fn knight_probability(n: i32, k: i32, row: i32, col: i32) -> f64 {
    if row < 0 || col < 0 || row >= n || col >= n {
        return 0.0;
    }
    if k == 0 {
        return 1.0;
    }
    let mut res = 0f64;
    for d in knight_directions() {
        res += knight_probability(n, k - 1, row + d[0], col + d[1]) / 8.0;
    }
    res
}

fn knight_probability_memo(n: i32, k: i32, row: i32, col: i32) -> f64 {
    fn __knight_probability(
        n: i32,
        k: i32,
        row: i32,
        col: i32,
        dp: &mut Vec<Vec<Vec<f64>>>,
    ) -> f64 {
        if row < 0 || col < 0 || row >= n || col >= n {
            return 0.0;
        }
        if k == 0 {
            return 1.0;
        }
        if dp[k as usize][row as usize][col as usize] != -1.0 {
            return dp[k as usize][row as usize][col as usize];
        }

        let mut res = 0f64;
        for d in knight_directions() {
            res += knight_probability(n, k - 1, row + d[0], col + d[1]) / 8.0;
        }
        dp[k as usize][row as usize][col as usize] = res;
        res
    }

    let mut dp = vec![vec![vec![-1.0; n as usize]; n as usize]; (k + 1) as usize];
    __knight_probability(n, k, row, col, &mut dp)
}

fn knight_probability_bottom_up(n: i32, k: i32, row: i32, col: i32) -> f64 {
    let mut dp = vec![vec![vec![0.0; n as usize]; n as usize]; (k + 1) as usize];
    dp[0][row as usize][col as usize] = 1.0;
    for step in 1..=k {
        for row in 0..n {
            for col in 0..n {
                for d in knight_directions() {
                    let (prev_row, prev_col) = (row + d[0], col + d[1]);
                    if prev_row >= 0 && prev_row < n && prev_col >= 0 && prev_col < n {
                        dp[step as usize][row as usize][col as usize] +=
                            dp[(step - 1) as usize][prev_row as usize][prev_col as usize] / 8.0;
                    }
                }
            }
        }
    }
    let mut res = 0.0;
    for i in 0..n {
        for j in 0..n {
            res += dp[k as usize][i as usize][j as usize];
        }
    }
    res
}

fn knight_probability_bottom_up_opt(n: i32, k: i32, row: i32, col: i32) -> f64 {
    let mut dp_prev = vec![vec![0.0; n as usize]; n as usize];
    let mut dp_next = vec![vec![0.0; n as usize]; n as usize];

    dp_prev[row as usize][col as usize] = 1.0;
    for step in 1..=k {
        for row in 0..n {
            for col in 0..n {
                for d in knight_directions() {
                    let (prev_row, prev_col) = (row + d[0], col + d[1]);
                    if prev_row >= 0 && prev_row < n && prev_col >= 0 && prev_col < n {
                        dp_next[row as usize][col as usize] +=
                            dp_prev[prev_row as usize][prev_col as usize] / 8.0;
                    }
                }
            }
        }
        dp_prev = dp_next;
        dp_next = vec![vec![0.0; n as usize]; n as usize];
    }

    let mut res = 0.0;
    for i in 0..n {
        for j in 0..n {
            res += dp_prev[i as usize][j as usize];
        }
    }
    res
}

fn lcs(s1: String, s2: String) -> i32 {
    let mut dp =  vec![vec![0u16; s2.len()+1]; s1.len()+1];
    let (b1, b2) = (s1.into_bytes(), s2.into_bytes());
    for i in 0..b1.len() {
      for j in 0..b2.len() {
        dp[i+1][j+1] = if b1[i] == b2[j] { dp[i][j]+1 } else { std::cmp::max(dp[i+1][j], dp[i][j+1]) }
      }
    }
    return dp[b1.len()][b2.len()] as i32;
}
#[cfg(test)]
mod test {
    use super::*;

    // https://leetcode.com/problems/min-cost-climbing-stairs/

    #[test]
    fn min_cost_climbing_stairs_top_bottom_test() {
        assert_eq!(15, min_cost_climbing_stairs_top_bottom(vec![10, 15, 20]))
    }

    #[test]
    fn min_cost_climbing_stairs_top_bottom_memo_test() {
        assert_eq!(
            15,
            min_cost_climbing_stairs_top_bottom_memo(vec![10, 15, 20])
        )
    }

    #[test]
    fn min_cost_climbing_stairs_top_bottom_memo_opt_test() {
        assert_eq!(
            15,
            min_cost_climbing_stairs_top_bottom_memo_opt(vec![10, 15, 20])
        )
    }

    // https://leetcode.com/problems/knight-probability-in-chessboard/
    #[test]
    fn knight_probability_test() {
        assert_eq!(1.0, knight_probability(1, 0, 0, 0));
        assert_eq!(0.06250, knight_probability(3, 2, 0, 0));
    }
    #[test]
    fn knight_probability_memo_test() {
        assert_eq!(0.06250, knight_probability_memo(3, 2, 0, 0));
    }
    #[test]
    fn knight_probability_bottom_up_test() {
        assert_eq!(0.06250, knight_probability_bottom_up(3, 2, 0, 0));
    }
    #[test]
    fn knight_probability_bottom_up_opt_test() {
        assert_eq!(0.06250, knight_probability_bottom_up_opt(3, 2, 0, 0));
    }

    // LCS
    // https://leetcode.com/problems/longest-common-subsequence

    #[test]
    fn lcs_test() {
      assert_eq!(3, lcs("abcde".to_string(), "ace".to_string()));
      assert_eq!(1, lcs("abc".to_string(), "dbf".to_string()));
  }
}
