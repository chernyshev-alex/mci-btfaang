use super::*;
use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

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

// ==== list allowed cycle ===== 
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        Self { next: None, val }
    }

    fn list_detect_cycle_floyd(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
        let (mut tortoise, mut hare) = (head.borrow().clone(), head.borrow().clone());
        loop {
            if tortoise.is_none() || hare.is_none() {
                // TODO rem dups
                return None;
            }
            tortoise = tortoise.unwrap().borrow_mut().next.clone();
            hare = hare.unwrap().borrow_mut().next.clone();

            if tortoise.is_none() || hare.is_none() {
                return None;
            } else {
                hare = hare.unwrap().borrow_mut().next.clone();
            }
            if tortoise == hare {
                break;
            }
        }
        let mut begin = head.borrow().clone();
        while begin != tortoise {
            begin = begin.unwrap().borrow_mut().next.clone();
            tortoise = tortoise.unwrap().borrow_mut().next.clone();
        }
        tortoise
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use std::{borrow::BorrowMut};
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
        let v = [-4, 0, 2, 3];
        // reverse & link nodes
        let mut ls = v.iter()
            .map(|e| Rc::new(RefCell::new(ListNode::new(*e))))
            .reduce(|acc,  el| {
               el.replace_with(|a| {
                let mut e1 = ListNode::new(a.val);
                e1.next = Some(acc);
                e1 });
                return el;
        });

        //  make cycle  -4 -> 2
        let mut curr = ls.borrow_mut().clone();
        let mut ref_to = curr.clone();
        while let Some(ref mut node) = curr.clone() {
          let mut v = node.as_ref().borrow_mut();
          if v.val == 2 {
            ref_to = Some(node.clone()); 
          }
          if v.val == -4  {
            v.next = ref_to.clone();
            break;
          }
          curr = v.next.clone();
        }
        let result = ListNode::list_detect_cycle_floyd(ls);
        let cycle_node = result.unwrap().as_ref().borrow().val;
        assert_eq!(2, cycle_node);
    }
}
