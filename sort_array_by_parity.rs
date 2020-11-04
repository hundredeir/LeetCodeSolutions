/*
https://leetcode.com/problems/sort-array-by-parity/
*/

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut arr = a;
        let mut begin = 0;
        let len = arr.len();
        let mut end = len - 1;
        
        while true {
            while begin < len && arr[begin] % 2 == 0 { begin += 1;}
            while end  > 0 && arr[end] % 2 != 0 { end -= 1;}
            if begin >= end {break;}
            let temp = arr[end];
            arr[end] = arr[begin];
            arr[begin] = temp;
        }
        arr
    }
}