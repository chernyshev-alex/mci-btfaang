use std::borrow::Borrow;
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

pub fn right_side_view_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, level: usize, v:  &mut Vec<i32>) -> &Vec<i32> {
        if let Some(n) = node {
            let x = n.as_ref().borrow();
            if level >= v.len() {
                v.push(x.val);
            }
            dfs(x.right.clone(), level + 1, v);
            dfs(x.left.clone(), level + 1, v);
        } 
        v
    }
    dfs(root, 0, &mut vec![]).to_vec()
}

pub fn count_nodes_rec(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() { return 0; }
    return count_nodes_rec(root.clone().unwrap().as_ref().borrow().left.clone())
            + count_nodes_rec(root.clone().unwrap().as_ref().borrow().right.clone())
            + 1
}

pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn tree_height(mut node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut h = 0;
        while let Some(rc) = node {
            let b = rc.as_ref().borrow();
            if b.left.is_some() {
                h +=1;
            }
            node = b.left.clone();
        }
        h
    }

    fn node_exists(idx: i32, h: i32, mut node: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (mut l , mut r , mut count) = (0, 2_i32.pow(h as u32) -1, 0);
        while count < h  {
            let mid = f32::ceil((l+r) as f32 / 2.0) as i32;
            if let Some(n) = node {
                let b  = n.as_ref().borrow();
                if idx  >= mid {
                    node = b.right.clone(); 
                    l = mid;
                } else {
                    node = b.left.clone();  
                    r  = mid -1;
                }
            }
            count += 1;     
        }
        return node.is_some();
    }

    if root.is_none() {
        return 0;
    }

    let src_root = root.clone();

    let h = tree_height(root);
    if h  == 0 { return  1 } 
    
    let upper_count = 2_i32.pow(h as u32) -1;
    let (mut l, mut r) = (0, upper_count);
    while l < r {
        let idx_find = f32::ceil((l+r) as f32 / 2.0) as i32;
        if node_exists(idx_find , h, src_root.clone()) {
            l = idx_find;
        } else {
            r = idx_find -1;
        }
    }
    return upper_count + l + 1;
}

pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    false
}

#[cfg(test)]

    fn leetcode_level_order_tree() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
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
        })))
    }

    fn leetcode_tree_test() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
        })))
    }

    fn leetcode_tree_test2() -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
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
        })))
    }

    fn valid_tree() -> Option<Rc<RefCell<TreeNode>>> { 
        Some(Rc::new(RefCell::new(TreeNode { val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None, }))),
            right: Some(Rc::new(RefCell::new(TreeNode { val: 3, left: None, right: None, }))),
        })))
    }

    fn count_tree_test() -> Option<Rc<RefCell<TreeNode>>> { 
        Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })))
    }

mod test {
    use super::*;

    // https://leetcode.com/problems/maximum-depth-of-binary-tree/
    #[test]
    fn max_depth_test() {
        assert_eq!(3, max_depth(leetcode_tree_test2()));
    }

    // https://leetcode.com/problems/binary-tree-level-order-traversal/
    #[test]
    fn level_order_test() {
        assert_eq!(vec![vec![3], vec![9, 20], vec![15, 7]], level_order(leetcode_level_order_tree()));
    }

    // https://leetcode.com/problems/binary-tree-right-side-view/
    #[test]
    fn right_side_view_bfs_test() {
        let tree = None;
        assert_eq!(vec![] as Vec<i32>, right_side_view_bfs(tree));

        assert_eq!(vec![1, 3, 4], right_side_view_bfs(leetcode_tree_test()));

        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(vec![1, 3], right_side_view_bfs(tree));
    }

    // https://leetcode.com/problems/binary-tree-right-side-view/
    #[test]
    fn right_side_view_dfs_test() {
        assert_eq!(vec![1, 3, 4], right_side_view_dfs(leetcode_tree_test()));
    }

    // https://leetcode.com/problems/count-complete-tree-nodes/
    #[test]
    fn count_nodes_rec_test() {
        assert_eq!(6, count_nodes_rec(count_tree_test()));
    }

    // https://leetcode.com/problems/count-complete-tree-nodes/
    #[test]
    fn count_nodes_test() {
        assert_eq!(6, count_nodes(count_tree_test()));
    }

    // https://leetcode.com/problems/validate-binary-search-tree/
    #[test]
    fn is_valid_bst_test() {
        assert_eq!(true, is_valid_bst(valid_tree()));
    }
}
