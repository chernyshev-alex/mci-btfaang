use std::str;

pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {

    fn partiton(v: &mut Vec<i32>, l: i32, r: i32) -> i32 {
        let pvt = v[r as usize];
        let mut pidx = l;
        for j in l..r {
            if v[j  as usize] <= pvt {
                v.swap(pidx as usize, j as usize);
                pidx += 1;
            }
        }
        v.swap(pidx as usize, r as usize);
        pidx
    }

    fn quick_sort(v: &mut Vec<i32>, l: i32, r: i32) {
        if l < r {
            let p = partiton(v, l, r);
            quick_sort(v, l, p -1 );
            quick_sort(v, p + 1, r);
        }
    }
    let r = nums.len() - 1;
    quick_sort(&mut nums, 0, r as i32);
    nums[nums.len() - k as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    // https://leetcode.com/problems/kth-largest-element-in-an-array/
    #[test]
    fn find_kth_largest_test() {
        assert_eq!(5, find_kth_largest(vec![3,2,1,5,6,4], 2));
    }
}
