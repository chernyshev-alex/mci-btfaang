use std::collections::{VecDeque, HashSet};

fn traversal_bfs(m : Vec<Vec<i32>>) -> Vec<i32> {
    let (mut seen, mut values) = (HashSet::<i32>::new(), vec![0;0]);
    let mut q = VecDeque::<i32>::new();
    q.push_back(0);
    while let Some(v) = q.pop_front() {
        values.push(v);
        seen.insert(v);
        for n in &m[v as usize] {
            if !seen.contains(&*n) {
                q.push_back(*n);
            }
        }
    }
    values
}

fn traversal_dfs(v: i32, m : &Vec<Vec<i32>>, values: &mut Vec<i32>, seen : &mut HashSet<i32>) {
    values.push(v);
    seen.insert(v);
    for n in &m[v as usize] {
        if !seen.contains(&*n) {
          traversal_dfs(*n, m, values, seen);
        }
    }
}

fn traversal_bfs_mx(m : Vec<Vec<i8>>) -> Vec<i8> {
    let (mut seen, mut values) = (HashSet::<i8>::new(), Vec::<i8>::new());
    let mut q = VecDeque::<i8>::new();
    q.push_back(0);
    while let Some(v) = q.pop_front() {
      values.push(v);
      seen.insert(v);
      for (v, is_connected) in m[v as usize].iter().enumerate() {
          if *is_connected >0 && !seen.contains(&(v as i8)) {
              q.push_back(v as i8);
          }
      }
    }
    values
}

fn traversal_dfs_mx(v: i8, m : &Vec<Vec<i8>>, values: &mut Vec<i8>, seen : &mut HashSet<i8>) {
  values.push(v);
  seen.insert(v);
  for (v, is_connected) in m[v as usize].iter().enumerate() {
      if *is_connected >0 && !seen.contains(&(v as i8)) {
        traversal_dfs_mx(v as i8, m, values, seen);
      }
  }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn traversal_bfs_test() {
        let adjacency_list = vec![
            vec![1, 3],
            vec![0],
            vec![3, 8],
            vec![0, 2, 4, 5],
            vec![3, 6],
            vec![3],
            vec![4, 7],
            vec![6],
            vec![2]];

        assert_eq!(vec![0, 1, 3, 2, 4, 5, 8, 6, 7], traversal_bfs(adjacency_list));
    }

    #[test]
    fn traversal_dfs_test() {
        let adjacency_list = vec![
            vec![1, 3],
            vec![0],
            vec![3, 8],
            vec![0, 2, 4, 5],
            vec![3, 6],
            vec![3],
            vec![4, 7],
            vec![6],
            vec![2]];

        let mut values = vec![];
        traversal_dfs(0, &adjacency_list, &mut values, &mut HashSet::new());
        assert_eq!(vec![0, 1, 3, 2, 8, 4, 6, 7, 5], values);
    }

    #[test]
    fn traversal_bfs_mx_test() {
        let adjacency_matrix: Vec<Vec<i8>> = vec![
            vec![0, 1, 0, 1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0, 0]];

        assert_eq!(vec![0, 1, 3, 2, 4, 5, 8, 6, 7], traversal_bfs_mx(adjacency_matrix));
    }

    #[test]
    fn traversal_dfs_mx_test() {
        let adjacency_matrix = vec![
            vec![0, 1, 0, 1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 1, 0],
            vec![0, 0, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 0, 0]];

        let mut values = vec![];
        traversal_dfs_mx(0, &adjacency_matrix, &mut values, &mut HashSet::new());
        assert_eq!(vec![0, 1, 3, 2, 8, 4, 6, 7, 5], values);
    }
}
