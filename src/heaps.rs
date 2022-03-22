
pub struct MaxHeap {
    data: Vec<i32>,
}

impl MaxHeap {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn peek(&self) -> Option<&i32> {
        self.data.get(0)
    }

    pub fn push(&mut self, v : i32) {
        self.data.push(v);
        self.sift_up();
    }

    #[inline]
    fn parent_index(index : usize) -> usize  {
        f32::floor((index -1) as f32 / 2.0) as usize
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }

    fn compare(&self, i : usize, j : usize) -> i32  {
        self.data[i] -  self.data[j]
    }

    fn sift_up(&mut self) {
        let mut node_idx = self.data.len() -1;
        while node_idx > 0 && self.compare(node_idx, Self::parent_index(node_idx)) > 0 {
            self.data.swap(node_idx, Self::parent_index(node_idx));
            node_idx = Self::parent_index(node_idx);
        }   
    }
}

#[cfg(test)]
mod test {
use super::*;
#[test]
fn min_heap_test() {
    let mut heap = MaxHeap::new();
    heap.push(20);
    heap.push(22);
    heap.push(1);
    heap.push(5);

    assert_eq!(heap.peek(), Some(&22));
  
  }
}