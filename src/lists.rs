
use std::collections::LinkedList;
use std::ptr;

// --------- Simple Enum based list --------------- //

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

// --------- simple node based list  --------------- //

#[derive(Debug)]
struct Node<T> {
    pub val: T,
    pub next: *mut Node<T>,
}
#[derive(Debug)]
struct NodeList<T> {
    pub head: *mut Node<T>,
    pub tail: *mut Node<T>,
}
pub struct IterNodeList<T> {
    head: *mut Node<T>,
    curr: *mut Node<T>,
    tail: *mut Node<T>,
}

impl<T> Drop for NodeList<T> {
    fn drop(&mut self) {
        let mut curr = self.head;
        while !curr.is_null() {
            let next: *mut Node<T> = unsafe { (*curr).next };
            unsafe {
                Box::from_raw(curr);
            }
            curr = next;
        }
    }
}

impl<T: Copy> Iterator for IterNodeList<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.curr.is_null() {
            return None;
        }
        let result = unsafe { &(*self.curr).val };
        self.curr = unsafe { (*self.curr).next };
        Some(*result)
    }
}

impl<T: Copy> NodeList<T> {
    pub fn iter(&self) -> IterNodeList<T> {
        IterNodeList {
            head: self.head,
            curr: self.head,
            tail: self.tail,
        }
    }

    pub fn reverse(&mut self) -> &Self {
        unsafe {
            let (mut p1, mut p2) = (self.head, (*self.head).next);
            (*p1).next = ptr::null_mut();
            while !p2.is_null() {
                let tmp_ref = (*p2).next;
                (*p2).next = p1;
                p1 = p2;
                p2 = tmp_ref;
            }
            self.tail = self.head;
            self.head = p1;
        }
        self
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut result = Vec::<T>::new();
        for el in self.iter() {
            result.push(el);
        }
        result
    }
}

impl<T: Copy, const N: usize> From<[T; N]> for NodeList<T> {
    fn from(arr: [T; N]) -> Self {
        let (mut head, mut tail) = (ptr::null_mut::<Node<T>>(), ptr::null_mut::<Node<T>>());
        for e in arr {
            let node_ptr = Box::into_raw(Box::new(Node {
                val: e,
                next: ptr::null_mut(),
            }));
            if head.is_null() {
                head = node_ptr;
                tail = head;
            } else {
                unsafe {
                    (*tail).next = node_ptr;
                    tail = (*tail).next;
                }
            }
        }
        NodeList { head, tail }
    }
}

//  -----  Linked list  ---- //

fn reverse_list_rec<T>(mut ls: LinkedList<T>, mut out: LinkedList<T>) -> LinkedList<T> {
    if ls.is_empty() {
        return out;
    }
    out.push_front(ls.pop_front().unwrap());
    reverse_list_rec(ls, out)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn elist_reverse() {
        let ls = EList::Cons(1, Box::new(EList::Cons(2, Box::new(EList::Nil))));
        assert_eq!("(2,(1,Nil))", ls.reverse().to_string())
    }

    #[test]
    fn node_list_reverse() {
        let mut ls = NodeList::from([1, 2, 3]);
        assert_eq!(vec![3, 2, 1], ls.reverse().to_vec())
    }

    #[test]
    fn linked_list_reverse_recursive() {
        let ls = LinkedList::from([1, 2, 3, 4]);
        let target = reverse_list_rec(ls, LinkedList::new());

        let res = target.iter().fold(vec![], |mut v: Vec<_>, e| {
            v.push(*e);
            v
        });
        assert_eq!(res.as_slice(), [4, 3, 2, 1]);
    }

    #[test]
    fn linked_list_reverse_rfold() {
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
    fn deque_reverse() {
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
