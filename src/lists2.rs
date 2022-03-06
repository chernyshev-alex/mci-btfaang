use super::*;
use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(i32, i32);
#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }

    //
    // https://leetcode.com/problems/linked-list-cycle-ii/
    //
    pub fn detect_cycle(&mut self) -> (bool, Option<Edge>) {
        let mut hs = HashSet::<i32>::new();
        let mut it = self.edges.iter_mut();
        while let Some(Edge(a, b)) = it.next() {
          if !hs.is_empty() && (hs.contains(a) || hs.contains(b)) {
                  return (true, Some(Edge(*a, *b)));
          } else {
              hs.insert(*a);
              hs.insert(*b);
          }
        }
        (false, None)
    }
}

impl From<i32> for Vertex {
    fn from(item: i32) -> Self {
        Vertex(item)
    }
}

impl From<(i32, i32)> for Edge {
    fn from(item: (i32, i32)) -> Self {
        Edge(item.0, item.1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;
    //
    // https://leetcode.com/problems/linked-list-cycle-ii/
    //
    #[test]
    fn list_detect_cycle_with_set () {
        let vertices = vec![1, 2];
        let edges = vec![(1, 2), (2, 1)];
        let mut g = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect());
        
        let result  = g.detect_cycle();
        assert_eq!(result, (true, Some(Edge(2,1))));
       
    }
}
