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

// ====
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn list_detect_cycle_floyd(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut tortoise, mut hare) = (head.as_ref(), head.as_ref());
        loop {
            tortoise = tortoise.unwrap().next.as_ref();
            hare = hare.unwrap().next.as_ref();

            if tortoise.is_none() || hare.is_none() {
                return None;
            } else {
                hare = hare.unwrap().next.as_ref();
            }
            if tortoise == hare {
                break;
            }
        }
        let (mut p1, mut p2) = (head.as_ref(), tortoise);
        while p1 != p2 {
            p1 = p1.unwrap().next.as_ref();
            p2 = p2.unwrap().next.as_ref();
        }
        Some(Box::new(ListNode::new(p2.unwrap().val)))
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::{
        collections::VecDeque,
        ops::DerefMut,
        ptr::{self, null},
    };
    //
    // https://leetcode.com/problems/linked-list-cycle-ii/
    //
    #[test]
    fn list_detect_cycle_with_set() {
        let vertices = vec![1, 2];
        let edges = vec![(1, 2), (2, 1)];
        let mut g = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        let result = g.detect_cycle();
        assert_eq!(result, (true, Some(Edge(2, 1))));
    }

    #[test]
    fn list_detect_cycle_floyd() {
        let mut v = [-4, 0, 2, 3];
        let mut ls = v.iter_mut().map(|e| Box::new(ListNode::new(*e))).reduce(
            |a: Box<ListNode>, mut e: Box<ListNode>| -> Box<ListNode> {
                e.next = Some(a);
                e
            },
        );
        // TODO : make cycle 

        ListNode::list_detect_cycle_floyd(ls);
    }
}
