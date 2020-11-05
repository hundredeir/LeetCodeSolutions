/*
https://leetcode.com/problems/3sum/
*/
use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let nums = nums;
        
        let mut set = HashSet::new();
        let mut ret:Vec<Vec<i32>> = vec![];
        
        for (i, m) in nums.iter().enumerate() {
            for (j, n) in nums[i+1..].iter().enumerate() {
                let req_num = 0 - m - n;
                let index = i+j+2;
                if let Ok(ind) = nums[index..].binary_search(&req_num) {
                    if !set.contains(&(*m, *n)) {
                        set.insert((*m, *n));
                        ret.push(vec![*m, *n, nums[index+ind]]);
                    }
                }
                
            }
        }
        ret
    }
}