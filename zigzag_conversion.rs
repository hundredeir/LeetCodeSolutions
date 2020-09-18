/*
https://leetcode.com/problems/zigzag-conversion/
*/

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 0 { return String::from("")}
        else if num_rows == 1 { return s;}
        let mut result = Vec::new();
        let char_arr:Vec<char> = s.chars().collect();
        for i in 0..num_rows {
            let mut j = 0;
            loop {
                if !(j == 0 || i == 0 || i == num_rows - 1) {
                    let index = j * (2 * (num_rows - 1)) - i;
                    if let Some(val) = char_arr.get(index as usize) {
                        result.push(*val);
                    }
                    else {
                        break;
                    }
                }
                
                let index = j * (2 * (num_rows - 1)) + i;
                if let Some(val) = char_arr.get(index as usize) {
                    result.push(*val);
                }
                else {
                    break;
                }
                
                j += 1;
            }
        }
        result.into_iter().collect()
    }
}