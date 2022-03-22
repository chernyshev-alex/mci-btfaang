use std::collections::VecDeque;

fn directions() -> Vec<Vec<i32>> {
    vec![vec![-1, 0], vec![0, 1], vec![1, 0], vec![0, -1]]
}

fn traversal_dfs(v: &Vec<Vec<i32>>) {
    fn dfs(
        m: &Vec<Vec<i32>>,
        row: i32,
        col: i32,
        visited: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if row < 0
            || col < 0
            || row >= m.len() as i32
            || col >= m[0].len() as i32
            || visited[row as usize][col as usize] == 1
        {
            return;
        }

        visited[row as usize][col as usize] = 1;
        path.push(m[row as usize][col as usize]);
        for d in directions() {
            dfs(m, row + d[0], col + d[1], visited, path);
        }
    }

    let mut path: Vec<i32> = vec![];
    let mut visited = v.clone();
    dfs(v, 0, 0, &mut visited, &mut path);
}

fn traversal_bfs(m: &Vec<Vec<i32>>) {
    let mut path: Vec<i32> = vec![];
    let mut visited = m.clone();
    let mut q = VecDeque::<(i32, i32)>::new();

    q.push_back((0, 0));
    while let Some((row, col)) = q.pop_front() {
      if row < 0
        || col < 0
        || row >= m.len() as i32
        || col >= m[0].len() as i32
        || visited[row as usize][col as usize] == 1
      {
          continue;
      }

      visited[row as usize][col as usize] = 1;
      path.push(m[row as usize][col as usize]);
      for d in directions() {
        q.push_back((row + d[0], col + d[1]));
      }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_matrix() -> Vec<Vec<i32>> {
      let mut v: Vec<Vec<i32>> = vec![vec![0_i32; 5]; 5];
      for i in 0..v.len() {
          for j in 0..v[i].len() {
              v[i][j] = (v.len() * i) as i32 + j as i32;
          }
      }
      v
    }

    #[test]
    fn traversal_dfs_test() {
      traversal_dfs(&make_matrix());
    }

    #[test]
    fn traversal_bfs_test() {
      traversal_bfs(&make_matrix());
    }
}
