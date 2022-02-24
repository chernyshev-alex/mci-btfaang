use std::collections::LinkedList;
use std::collections::VecDeque;
use std::ops::Deref;
use std::ptr;
use std::ptr::NonNull;
use std::rc::Rc;

// --------- Simple enum List impl --------------- //

#[derive(Debug)]
enum EList<T> {
    Cons(T, Box<EList<T>>),
    Nil,
}

impl<T: Copy> EList<T> {
    fn reverse(&self) -> Box<EList<T>> {
        fn rev<T: Copy>(v: T, e: &EList<T>) -> Box<EList<T>> {
            match e {
                EList::Nil => Box::new(EList::Cons(v, Box::new(EList::Nil))),
                EList::Cons(a, tail) => Box::new(EList::Cons(*a, rev(v, tail))),
            }
        }

        match self {
            EList::Cons(v, tail) => rev(*v, tail.as_ref()),
            _ => Box::new(EList::Nil),
        }
    }
}

impl<T: std::fmt::Debug> ToString for EList<T> {
    fn to_string(&self) -> String {
        match self {
            EList::Cons(v, ls) => format!("({:?},{})", v, ls.to_string()),
            EList::Nil => format!("Nil"),
        }
    }
}

// --------- Node List impl --------------- //

#[derive(Debug)]
struct Node<T> {
    pub val: Option<T>,
    pub next: NonNull<Node<T>>,
}

// https://doc.rust-lang.org/std/primitive.pointer.html
// https://stackoverflow.com/questions/30831037/situations-where-cell-or-refcell-is-the-best-choice

impl<T, const N: usize> From<[T; N]> for Node<T> {
    fn from(arr: [T; N]) -> Self {
        let mut head: *mut Node<T> = ptr::null_mut();
        let mut last = head;
        for e in arr {
            let mut n = Box::new(Node {
                val: Some(e),
                next: NonNull::dangling(),
            });
            println!("iter ");
            if head.is_null() {
                head = &mut *n;
                println!("is null {:?}", head);
                last = head;
            } else {
                unsafe {
                    (*last).next = NonNull::new_unchecked(n.as_mut());
                }
                last = &mut *n;
                println!("head not null {:?} last {:?}", head, last);
            }
        }
        println!("return {:?} last {:?}", head, last);
        unsafe { std::ptr::read(head) }
    }
}

//1 -> 2 -> 3 -> None  =>  X -> 1 -> None
// pub fn reverse(self) -> Self {
//     let (mut curr, mut rev_head) = (self, ListNode::new(None));
//     while curr.next.is_some() {
//         let head = curr.next;
//         curr.next = rev_head.next;
//         rev_head = curr;
//         curr = *head.unwrap()
//     }
//     curr
// }

pub struct Solution;

impl Solution {
    pub fn reverse_list_rec<T>(mut ls: LinkedList<T>, mut out: LinkedList<T>) -> LinkedList<T> {
        if ls.is_empty() {
            return out;
        }
        out.push_front(ls.pop_front().unwrap());
        Self::reverse_list_rec(ls, out)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::{VecDeque};

    #[test]
    fn elist_reverse() {
        let ls = EList::Cons(1, Box::new(EList::Cons(2, Box::new(EList::Nil))));
        assert_eq!("(2,(1,Nil))", ls.reverse().to_string())
    }

    #[test]
    fn reverse_custom_list() {
        let head = Node::from([1, 2, 3]);
        let mut ptr = &head;
        ptr = {
            let ref this = head.next;
            unsafe { &mut *this.as_ptr() }
        };
    }

    #[test]
    fn reverse_list_rec() {
        let ls = LinkedList::from([1, 2, 3, 4]);
        let target = Solution::reverse_list_rec(ls, LinkedList::new());

        let res = target.iter().fold(vec![], |mut v: Vec<_>, e| {
            v.push(*e);
            v
        });
        assert_eq!(res.as_slice(), [4, 3, 2, 1]);
    }

    #[test]
    fn reverse_list_rfold() {
        let ls = LinkedList::from([1, 2, 3, 4, 5]);

        let rf = |mut acc: LinkedList<_>, el| {
            acc.push_front(el);
            acc
        };
        let target: LinkedList<_> = ls.iter().rev().rfold(LinkedList::new(), rf);

        let res = target.iter().fold(vec![], |mut v: Vec<_>, e| {
            v.push(**e);
            v
        });
        assert_eq!(res.as_slice(), [5, 4, 3, 2, 1]);
    }

    #[test]
    fn reverse_deque() {
        let mut xs = VecDeque::from([1, 2, 3, 4]);
        let (mut l, mut r) = (0, xs.len() - 1);
        while l < r {
            xs.swap(l, r);
            l += 1;
            r -= 1;
        }
        let res = xs.iter().fold(Vec::new(), |mut v: Vec<_>, e| {
            v.push(*e);
            v
        });
        assert_eq!(res, [4, 3, 2, 1]);
    }
}
