/*
https://leetcode.com/problems/minimum-cost-for-tickets/
*/

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let first = if costs[0] > costs[1] { costs[1]  } else { costs[0] };
        let first = if first > costs[2] { costs[2]  } else { first };
        
        let mut left_min_vec:Vec<i32> = vec![first];
        
        for (i, val) in days[1..].iter().enumerate() {
            let res = days.binary_search(&(val - 6));
            let mut cost = left_min_vec[i] + costs[0];
            match res {
                Err(0) | Ok(0) => {
                    cost = if cost > costs[1] {costs[1]} else {cost};
                },
                Ok(x) | Err(x) => {
                    let new_cost = left_min_vec[x-1] + costs[1];
                    cost = if cost > new_cost { new_cost } else { cost };
                }
            }
            
            let res = days.binary_search(&(val - 29));
            match res {
                Err(0) | Ok(0) => {
                    cost = if cost > costs[2] {costs[2]} else {cost};
                },
                Ok(x) | Err(x) => {
                    let new_cost = left_min_vec[x-1] + costs[2];
                    cost = if cost > new_cost { new_cost } else { cost };
                }
            }
            left_min_vec.push(cost);
        } 
        
        left_min_vec.pop().unwrap()
        
    }
    
}