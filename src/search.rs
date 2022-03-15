use std::str;

pub fn find_kth_largest_qsort(mut nums: Vec<i32>, k: i32) -> i32 {
    fn partiton(v: &mut Vec<i32>, l: usize, r: usize) -> i32 {
        let pvt = v[r];
        let mut pidx = l;
        for j in l..r {
            if v[j] <= pvt {
                v.swap(pidx, j);
                pidx += 1;
            }
        }
        v.swap(pidx as usize, r as usize);
        pidx as i32
    }

    fn quick_sort(v: &mut Vec<i32>, l: i32, r: i32) {
        if l < r {
            let p = partiton(v, l as usize, r as usize);
            quick_sort(v, l, p -1 );
            quick_sort(v, p + 1, r);
        }
    }
    let r = nums.len() - 1;
    quick_sort(&mut nums, 0, r as i32);
    nums[nums.len() - k as usize]
}

pub fn find_kth_largest_qselect(mut nums: Vec<i32>, k: usize) -> i32 {
    fn partiton(v: &mut Vec<i32>, l: usize, r : usize) -> usize {
        let mut i = l;
        for j in l..=r {
            if v[j] <= v[r] {
                v.swap(i, j);
                i +=1;
            }
        }
        i - 1
    }

    fn quick_select(v: &mut Vec<i32>, l: usize, r: usize, k : usize) -> i32 {
        let p_idx = partiton(v, l, r);
        match p_idx {
            _ if k < p_idx  => quick_select(v, l, p_idx-1, k), 
            _ if k > p_idx  => quick_select(v, p_idx+1, r, k), 
            _  => v[k]
        }
    }

    let r = nums.len();
    quick_select(&mut nums, 0, r -1 , r- k)
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    fn binary_search(v: &Vec<i32>, mut l : i32, mut r : i32, target : i32) -> i32 {
        while l <= r {
            let mid  = (r + l) /2;
            let mid_val = v[mid as usize];
            match mid_val {
                _ if mid_val < target => l = mid +1,  
                _ if mid_val > target => r = mid -1,
                _  => return mid as i32
            }
        }
        -1
    }

    let (mut first_pos, mut second_pos) = (-1, -1);
    if nums.is_empty() {
        return vec![first_pos, second_pos];
    }
    if nums.len() == 1  {
        if nums[0] == target {
            return vec![0, 0];
        } else {
            return vec![-1, -1];
        }
    }

    let r = nums.len();
    first_pos = binary_search(&nums, 0, (r-1) as i32 , target);
    if first_pos == -1 {
        return vec![first_pos, second_pos];
    }   

    let (mut end_pos, mut start_pos) = (first_pos, first_pos);
    let (mut t1, mut t2) = (-1, -1);

    while start_pos != -1 {
        t1 = start_pos;
        if start_pos  > 0 {
            start_pos = binary_search(&nums, 0, start_pos -1, target);
        } else {
            start_pos = -1;
        }
    }
    start_pos = t1;

    while end_pos != -1 {
        t2 = end_pos;
        end_pos = binary_search(&nums,end_pos + 1 , (nums.len()-1) as i32, target);
    }
    end_pos =  t2;
    
    vec![start_pos, end_pos]
}

#[cfg(test)]
mod test {
    use super::*;

    // https://leetcode.com/problems/kth-largest-element-in-an-array/
    #[test]
    fn find_kth_largest_qsort_test() {
        assert_eq!(5, find_kth_largest_qsort(vec![3,2,1,5,6,4], 2));
    }

    // https://leetcode.com/problems/kth-largest-element-in-an-array/
    // quick select approach
    #[test]
    fn find_kth_largest_qselect_test() {
        assert_eq!(5, find_kth_largest_qselect(vec![3,2,1,5,6,4], 2));
    } 

    // https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
    // nlogN
    #[test]
    fn search_range_test() {
        assert_eq!(vec![-1, -1], search_range(vec![2,2], 1));
        assert_eq!(vec![-1, -1], search_range(vec![1], 0));
        assert_eq!(vec![3,4], search_range(vec![5,7,7,8,8,10], 8));
        assert_eq!(vec![1, 1], search_range(vec![1, 4], 4));
        assert_eq!(vec![0,0], search_range(vec![1, 3], 1));
        assert_eq!(vec![0,0], search_range(vec![1], 1));
        assert_eq!(vec![-1,-1], search_range(vec![5,7,7,8,8,10], 6));
        assert_eq!(vec![-1,-1], search_range(vec![], 0));
        assert_eq!(vec![0,1], search_range(vec![2,2], 2));
    } 
}
