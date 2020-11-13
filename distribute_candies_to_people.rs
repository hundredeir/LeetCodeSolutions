/*
https://leetcode.com/problems/distribute-candies-to-people/
*/
impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut x = 0;
        while Self::series_sum(1, 1, x) <= candies {
            x += 1;
        }
        let terms = x - 1;
        let rows = terms/num_people;
        let mut ret:Vec<i32> = vec![];       
        
        for i in 1..=num_people {
            let sum = Self::series_sum(i, num_people, rows);
            ret.push(sum);
        }
        
        let filled_terms = num_people * rows;
        let left_terms = terms - filled_terms;
        
        for i in 1..=left_terms {
            ret[(i as usize) - 1] += i + filled_terms;
        }
        let residue = candies - Self::series_sum(1,1,terms);
        if residue > 0 {
            ret[left_terms as usize] += residue;
        }
        
        
        ret
    }
    
    fn series_sum(first:i32, diff:i32, terms:i32) -> i32 {
        let last_term = first + (terms - 1) * diff;
        terms * (first + last_term) / 2
    }
}