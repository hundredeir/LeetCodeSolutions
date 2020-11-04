/*
https://leetcode.com/problems/goat-latin/
*/

use std::collections::HashSet;

impl Solution {
    pub fn to_goat_latin(s: String) -> String {
        let vowel_set: HashSet<&str> = vec!["a", "e", "i", "o", "u"].into_iter().collect();
        let words:Vec<&str> = s.split(' ').to_owned().collect();
        let mut ret = vec![];
        
        for (i, word) in words.into_iter().enumerate() {
            match word.chars().next().unwrap() {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'  => {
                    ret.push(word.to_owned() + "ma" + &str::repeat("a", i+1));
                }
                x => {
                    ret.push(word[1..].to_owned() + &x.to_string() + "ma" + &str::repeat("a", i+1));
                }
            }
        }
        
        let mut ret:String = ret.into_iter().map(move |_s| {_s + " "}).collect();
        ret.pop();
        ret
    }
}