/*
https://leetcode.com/problems/next-permutation/
*/
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut indx = nums.len() - 1;
        while indx != 0 && nums[indx - 1] >= nums[indx] {
            indx -= 1;
        }
        
        if indx != 0 {
            indx = indx - 1;
            let indx = indx;
            let mut max_indx = indx;
            for (offset, &num) in nums[indx..].iter().enumerate() {
               if num > nums[indx] {
                   max_indx = indx + offset;
                }
            }
            nums.swap(indx, max_indx);
            nums[indx+1..].reverse();
        }
        else {
            nums.reverse();
        }
    }
}