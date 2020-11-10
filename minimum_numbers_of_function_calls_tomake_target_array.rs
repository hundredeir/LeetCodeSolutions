/*
https://leetcode.com/problems/minimum-numbers-of-function-calls-to-make-target-array/
*/
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut max_log = 0;
        
        let mut ret = 0;
        
        for val in nums {
            ret += if val > 0 { 1 } else { 0 };
            
            let mut temp = val as u32;
            let mut log = 0;
            while temp > 1 {
                if temp & 0x01 == 1 {
                    ret += 1;
                }
                temp = temp >> 1;
                log += 1;
            }
            
            
            max_log = if log > max_log { log } else { max_log};
        }
        
        ret + max_log
    }
}