use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet, VecDeque},
};

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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm -----
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();
    let mut min_heap = BinaryHeap::new();

    dist[start] = 0;
    min_heap.push(State {
        cost: 0,
        position: start,
    });
    // min-heap
    while let Some(State { cost, position }) = min_heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < dist[next.position] {
                min_heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

fn num_islands(mut m: Vec<Vec<char>>) -> i32 {
    let mut result = 0;
    if m.len() == 0 {
        return result;
    }
    let mut q = VecDeque::new();
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] == '1' {
                result += 1;
                m[row][col] = '0';
                q.push_back((row, col));

                while let Some((r, c)) = q.pop_front() {
                    for d in directions() {
                        let (next_row, next_col) = (r as i32 + d[0], c as i32 + d[1]);

                        if next_row < 0
                            || next_col < 0
                            || next_row >= m.len() as i32
                            || next_col >= m[0].len() as i32
                        {
                            continue;
                        }

                        if m[next_row as usize][next_col as usize] == '1' {
                            q.push_back((next_row as usize, next_col as usize));
                            m[next_row as usize][next_col as usize] = '0';
                        }
                    }
                }
            }
        }
    }
    result
}

fn oranges_rotting(mut m: Vec<Vec<i32>>) -> i32 {
    if m.len() == 0 {
        return 0;
    }

    let mut freshOranges = 0;

    let mut rotten = VecDeque::<(usize, usize)>::new();
    for row in 0..m.len() {
        for col in 0..m[0].len() {
            match m[row][col] {
                2 => rotten.push_back((row, col)),
                1 => freshOranges += 1,
                _ => (),
            }
        }
    }

    let mut mins = 0;
    let mut currentQueueSize = rotten.len() as i32;

    while rotten.len() > 0 {
        if currentQueueSize == 0 {
            currentQueueSize = rotten.len() as i32;
            mins += 1;
        }

        if let Some((row, col)) = rotten.pop_front() {
            currentQueueSize -= 1;

            for d in directions() {
                let (next_row, next_col) = (row as i32 + d[0], col as i32 + d[1]);
                if next_row < 0
                    || next_col < 0
                    || next_row >= m.len() as i32
                    || next_col >= m[0].len() as i32
                {
                    continue;
                }
                if m[next_row as usize][next_col as usize] == 1 {
                    m[next_row as usize][next_col as usize] = 2;
                    freshOranges -= 1;
                    rotten.push_back((next_row as usize, next_col as usize));
                }
            }
        }
    }
    if freshOranges != 0 {
        return -1;
    }
    mins
}

fn walls_and_gates(m: &mut Vec<Vec<i32>>) {
    fn dfs(v: &mut Vec<Vec<i32>>, r: i32, c: i32, cur_value: i32) {
        if r < 0
            || c < 0
            || r >= v.len() as i32
            || c >= v[0].len() as i32
            || cur_value > v[r as usize][c as usize]
        {
            return;
        }
        v[r as usize][c as usize] = cur_value;
        for d in directions() {
            dfs(v, r as i32 + d[0], c as i32 + d[1], cur_value + 1);
        }
    }

    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] == 0 {
                dfs(m, r as i32, c as i32, 0);
            }
        }
    }
}

fn transpose(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut tr = vec![vec![0; m.len()]; m[0].len()];
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            tr[c][r] = m[r][c];
        }
    }
    tr
}

fn set_zeroes(m: &mut Vec<Vec<i32>>) {
    let (mut rows, mut cols) = (HashSet::<i32>::new(), HashSet::<i32>::new());
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if m[r][c] == 0 {
                rows.insert(r as i32);
                cols.insert(c as i32);
            }
        }
    }
    for r in 0..m.len() {
        for c in 0..m[0].len() {
            if rows.contains(&(r as i32)) || cols.contains(&(c as i32)) {
                m[r][c] = 0;
            }
        }
    }
}

fn search_matrix(m: Vec<Vec<i32>>, target: i32) -> bool {
  let (mut l, mut r) = (0, m.len() * m[0].len());
  while l <= r {
      let mid = (l + r) / 2;
      let (row, col) = (mid / m[0].len(), mid % m[0].len());
      if row >= m.len() || col >= m[0].len() {
        break;
      }
      if m[row][col] == target {
          return true;
      }
      if target < m[row][col] {
        if mid == 0 { break; }
          r = mid - 1;
      } else {
          l = mid + 1;
      }
  }
  false
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

    #[test]
    fn dijkstra_shortest_path() {
        let graph = vec![
            vec![Edge { node: 2, cost: 10 }, Edge { node: 1, cost: 1 }],
            vec![Edge { node: 3, cost: 2 }],
            vec![
                Edge { node: 1, cost: 2 },
                Edge { node: 3, cost: 3 },
                Edge { node: 4, cost: 1 },
            ],
            vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
            vec![],
        ];

        assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    }

    // https://leetcode.com/problems/number-of-islands/
    #[test]
    fn num_islands_test() {
        let m = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(3, num_islands(m));
    }

    // https://leetcode.com/problems/rotting-oranges/
    #[test]
    fn oranges_rotting_test() {
        let m = vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]];
        assert_eq!(4, oranges_rotting(m));
        let m = vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]];
        assert_eq!(-1, oranges_rotting(m));
        let m = vec![vec![0, 2]];
        assert_eq!(0, oranges_rotting(m));
    }

    #[test]
    // Given 2d array containing  -1 - walls, 0-gates, INF - empty room.
    // Fill each empty rooom with a number of steps to the nearest gate.
    // Leave INF if it is impossible to reach a gate.
    fn walls_and_gates_test() {
        const INF: i32 = 2147483647;
        let mut m = vec![
            vec![INF, -1, 0, INF],
            vec![INF, INF, INF, 0],
            vec![INF, -1, INF, -1],
            vec![0, -1, INF, INF],
        ];
        walls_and_gates(&mut m);
    }

    // https://leetcode.com/problems/transpose-matrix/

    #[test]
    fn transpose_test() {
        let mut m = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]],
            transpose(m)
        );
    }

    // https://leetcode.com/problems/set-matrix-zeroes/

    #[test]
    fn set_zeroes_test() {
        let mut m = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut m);
        assert_eq!(vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]], m);
        let mut m = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        set_zeroes(&mut m);
        assert_eq!(
            vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]],
            m
        );
    }

    // https://leetcode.com/problems/search-a-2d-matrix/

    #[test]
    fn search_matrix_test() {
        let m = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert_eq!(true, search_matrix(m, 23));
        let m = vec![vec![1, 2, 3, 4, 5]];
        assert_eq!(true, search_matrix(m, 4));
        let m = vec![vec![1]];
        assert_eq!(false, search_matrix(m, 2));
    }
}
