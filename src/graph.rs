use std::{collections::{VecDeque, HashSet, BinaryHeap}};

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

fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
  fn dfs(cur_id : usize, adj_ls: &Vec<Vec<i32>>, inform_time: &Vec<i32>) -> i32 {
    if adj_ls[cur_id].len() == 0 {
      return 0
    }
    let mut max_time =0;
    for el in &adj_ls[cur_id] {
      max_time = std::cmp::max(max_time, dfs(*el as usize, adj_ls, inform_time));
    }
    max_time + inform_time[cur_id]
  }

  let mut adj_ls = vec![vec![0_i32;0]; manager.len()]; 
  for (empl_id, mgr_id) in manager.iter().enumerate() {
    if *mgr_id != -1 {
      adj_ls[*mgr_id as usize].push(empl_id as i32);
    }
  }  
  dfs(head_id as usize, &adj_ls, &inform_time)
}

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
  let mut adj_lst = vec![vec![0_usize;0]; num_courses as usize];
  for p in prerequisites.iter() {
    adj_lst[p[1]as usize].push(p[0] as usize);  // p[0, <- 1]
  }

  for nm in 0..num_courses {
    let mut seen = vec![false; num_courses as usize];
    let mut q = VecDeque::<usize>::new();

    for course_id in  &adj_lst[nm as usize] {
      q.push_back(*course_id as usize);
    }

    while let Some(curr) = q.pop_front() {
      seen[curr] = true;
      if curr == (nm as usize) { return false; }

      for next in &adj_lst[curr] {
        if seen[*next] == false {
          q.push_back(*next);
        }
      }
    }
  }
  true
}

fn can_finish_adjlst_tsort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
  let mut in_degree = vec![0; num_courses as usize];
  let mut adj_lst = vec![vec![0_usize;0]; num_courses as usize];
  for p in prerequisites.iter() {
    adj_lst[p[1]as usize].push(p[0] as usize); 
    in_degree[p[0] as usize] += 1;
  }

  let mut stack = VecDeque::<usize>::new();
  for d in &in_degree {
    if *d == 0 {
      stack.push_front(*d);
    }
  }

  let mut count  =0;
  while let Some(curr) = stack.pop_front() {
    count +=1;
    for next in &adj_lst[curr] {
      in_degree[*next] -=1;
      if in_degree[*next] == 0 {
        stack.push_front(*next);
      }
    }
  }
  count == num_courses
}

fn can_finish_opt_tsort(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
  let mut in_degree = vec![0; num_courses as usize];
  for p in prerequisites.iter() {
    in_degree[p[0] as usize] += 1;
  }

  let mut stack = VecDeque::<usize>::new();
  for (index, value)  in in_degree.iter().enumerate() {
    if *value == 0 {
      stack.push_front(index);
    }
  }

  let mut count  =0;
  while let Some(curr) = stack.pop_front() {
    count +=1;
    for p in &prerequisites {
      if curr == p[1] as usize {
        let p0 = p[0] as usize;
        in_degree[p0] -= 1;
        if in_degree[p0] == 0 {
          stack.push_front(p0);
        }
      }  
    }
  }
  count == num_courses
}

fn network_delay_dejkstra_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
  let mut dist = vec![std::usize::MAX; n as usize];
  let mut adj_lst = vec![vec![(0usize, 0usize);0]; n as usize];
  let mut heap = BinaryHeap::<usize>::new();

  let from_idx = (k - 1) as usize;
  dist[from_idx] = 0;

  heap.push(from_idx);

  for t in times.iter() {
    let (from, to, w) = (t[0], t[1], t[2]);
    let mut v = vec![((to-1) as usize, w as usize)];
    adj_lst[(from-1) as usize].append(&mut v); 
  }

  while let Some(v_idx) = heap.pop() {
    for (to, w) in &adj_lst[v_idx] {
      if dist[v_idx] + *w < dist[*to] {
        dist[*to] = dist[v_idx as usize] + w;
        heap.push(*to);
      }     
    }
  }
  let res = dist.iter().max();
  res.map(|val| -> i32 {if *val == std::usize::MAX { -1 } else { *val as i32 }}).unwrap()
}

fn network_delay_time_belman_ford(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
  let mut dist = vec![std::i32::MAX; n as usize];
  dist[(k - 1) as usize] = 0;
  for i in 0..n-1 {
    let mut count = 0;
    for t in &times {
      let (from, to, w) = (t[0], t[1], t[2]);
      if dist[(from -1) as usize] + w < dist[to as usize] {
        dist[(to -1) as usize] = dist[(from -1) as usize] + w;
        count += 1;
      }
    }
    if count == 0 { break }
  }
  let res = dist.iter().max();
  res.map(|val| -> i32 { if *val == std::i32::MAX { -1 } else { *val } }).unwrap()
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

    // Time Needed to Inform All Employees
    // https://leetcode.com/problems/time-needed-to-inform-all-employees/

    #[test]
    fn num_of_minutes_test() {
      let result = num_of_minutes(1,0, vec![-1], vec![0]);
      assert_eq!(0, result);
      let result = num_of_minutes(6,2, vec![2,2,-1,2,2,2], vec![0,0,1,0,0,0]);
      assert_eq!(1, result);
    }

    // https://leetcode.com/problems/course-schedule/

    #[test]
    fn can_finish_test() {
      let result = can_finish(4, 
        vec![vec![2,0], vec![1,0], vec![3,1],vec![3,2],vec![1,3]]);
      assert_eq!(false, result);
      let result = can_finish(2, vec![vec![1,0]]);
      assert_eq!(true, result);
      let result = can_finish(2, vec![vec![1,0], vec![0,1]]);
      assert_eq!(false, result);
    }

    #[test]
    fn can_finish_tsort_adjlst_test() {
      let result = can_finish_adjlst_tsort(4, 
        vec![vec![2,0], vec![1,0], vec![3,1],vec![3,2],vec![1,3]]);
      assert_eq!(false, result);
      let result = can_finish_adjlst_tsort(2, vec![vec![1,0]]);
      assert_eq!(true, result);
      let result = can_finish_adjlst_tsort(2, vec![vec![1,0], vec![0,1]]);
      assert_eq!(false, result);
    }

    #[test]
    fn can_finish_tsort_opt_test() {
      let result = can_finish_opt_tsort(4, 
        vec![vec![2,0], vec![1,0], vec![3,1],vec![3,2],vec![1,3]]);
      assert_eq!(false, result);
      let result = can_finish_opt_tsort(2, vec![vec![1,0]]);
      assert_eq!(true, result);
      let result = can_finish_opt_tsort(2, vec![vec![1,0], vec![0,1]]);
      assert_eq!(false, result);
      let result = can_finish_opt_tsort(2, vec![vec![0,1]]);
      assert_eq!(true, result);
    }

    // https://leetcode.com/problems/network-delay-time/

    #[test] 
    fn network_delay_time_dejkstra_test() {
      let result = network_delay_dejkstra_time(vec![vec![2,1,1],vec![2,3,1],vec![3,4,1]], 4, 2);
      assert_eq!(2, result);
      let result = network_delay_dejkstra_time(vec![vec![1,2,1]], 2, 1);
      assert_eq!(1, result);
      let result = network_delay_dejkstra_time(vec![vec![1,2,1]], 2, 2);
      assert_eq!(-1, result);
    }

    #[test] 
    fn network_delay_time_belman_ford_test() {
      let v = vec![vec![1, 4, 2], vec![1, 2, 9], vec![4, 2, -4], 
          vec![2, 5, -3], vec![4, 5, 6],vec![3, 2, 3], vec![5, 3, 7], vec![3, 1, 5]];
      let result = network_delay_time_belman_ford(v, 5, 1);
      assert_eq!(2, result);
    }

}
