use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn max_depth_int(node: Option<&Rc<RefCell<TreeNode>>>, depth: i32) -> i32 {
        if let Some(cell) = node {
            std::cmp::max(
                max_depth_int(cell.as_ref().borrow().left.as_ref(), 1 + depth),
                max_depth_int(cell.as_ref().borrow().right.as_ref(), 1 + depth),
            )
        } else {
            depth
        }
    }
    max_depth_int(root.as_ref(), 0)
}

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    let mut q = &mut VecDeque::<Rc<RefCell<TreeNode>>>::new();

    if let Some(n) = root {
        q.push_front(n);
    }

    while q.len() > 0 {
        let mut level: Vec<i32> = vec![];
        let mut next_level: Vec<Rc<RefCell<TreeNode>>> = vec![];
        while let Some(n) = q.pop_front() {
            let x = n.as_ref().borrow();
            level.push(x.val);

            let (l, r) = (x.left.clone(), x.right.clone());
            if l.is_some() {
                next_level.push(l.unwrap());
            }
            if r.is_some() {
                next_level.push(r.unwrap());
            }
        }
        res.push(level);
        next_level.iter().for_each(|e| q.push_back(e.clone()));
    }
    res
}

pub fn right_side_view_bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let mut q = &mut VecDeque::<Rc<RefCell<TreeNode>>>::new();
    if let Some(n) = root {
        q.push_front(n);
    }
    let mut val = 0;
    while q.len() > 0 {
        let (mut cnt, qlen) = (0, q.len());
        while cnt < qlen {
            if let Some(n) = q.pop_front() {
                let x = n.as_ref().borrow();
                val = x.val;
                let (l, r) = (x.left.clone(), x.right.clone());
                if l.is_some() {
                    q.push_back(l.unwrap());
                }
                if r.is_some() {
                    q.push_back(r.unwrap());
                }
            }
            cnt += 1;
        }
        res.push(val);
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    // https://leetcode.com/problems/maximum-depth-of-binary-tree/
    #[test]
    fn max_depth_test() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(3, max_depth(tree));
    }

    // https://leetcode.com/problems/binary-tree-level-order-traversal/
    #[test]
    fn level_order_test() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], level_order(tree));
    }

    // https://leetcode.com/problems/binary-tree-right-side-view/
    #[test]
    fn right_side_view_test() {
        let tree = None;
        assert_eq!(vec![] as Vec<i32>, right_side_view_bfs(tree));

        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5, left: None, right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4, left: None, right: None,
                }))),
            }))),
        })));
        assert_eq!(vec![1,3,4], right_side_view_bfs(tree));

        let tree = Some(Rc::new(RefCell::new(TreeNode { 
            val: 1, left: None,
            right: Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None })))
            })));
        assert_eq!(vec![1,3], right_side_view_bfs(tree));
    }
}
