/*
https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array

Time complexities
Average time complexity: O(logn) 
Average space complexity: O(1)
*/
use std::cmp::Ordering;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let rhs_cmp = |&probe| {
            if probe <= target {return Ordering::Less;}
            else {return Ordering::Greater;}
        };
        let lhs_cmp = |&probe| {
            if probe >= target {return Ordering::Greater;}
            else {return Ordering::Less;}
        };
        
        let mut ret = vec![];
        let res = nums.binary_search(&target);
        if let Err(_) = res {
            return vec![-1, -1];
        }
        let res = nums.binary_search_by(lhs_cmp);
        if let Err(val) = res {
            ret.push(val as i32)
        }
        let res = nums.binary_search_by(rhs_cmp);
        if let Err(val) = res {
            //Use of -1 is necessary. It because of how Rust's binary search does scalin operation
            ret.push(-1 + val as i32) 
        }
        ret
    }
    
}