use std::{collections::{BinaryHeap, VecDeque}, cmp::Ordering};

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
     other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
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
    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start, });
    // min-heap
    while let Some(State { cost, position }) = heap.pop() {
      if position == goal { return Some(cost); }
      if cost > dist[position] { continue; }

      for edge in &adj_list[position] {
        let next = State { cost: cost + edge.cost, position: edge.node };
        
        if next.cost < dist[next.position] {
          heap.push(next);
          dist[next.position] = next.cost;
        }
      }
    }
    None
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
          vec![Edge { node: 1, cost: 1 }, Edge { node: 3, cost: 3 }, Edge { node: 4, cost: 1 },],
          vec![Edge { node: 0, cost: 7 }, Edge { node: 4, cost: 2 }],
          vec![],];

      assert_eq!(shortest_path(&graph, 0, 4), Some(5));
    }
}
