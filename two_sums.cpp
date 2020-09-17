/*
https://leetcode.com/problems/two-sum/
*/

#include <iterator>
#include <unordered_set>
#include <set>
#include <vector>
#include <stdexcept>


class Solution {
public:
    vector<int> twoSum(vector<int>& nums, int target) {
        std::unordered_multiset<int> var_set(nums.size());
        std::unordered_multiset<int>::const_iterator val;
        for(std::vector<int>::iterator iter = nums.begin(); iter != nums.end(); ++iter)
        {
            val = var_set.find(target - *iter);
            if(val == var_set.end())
            {
                var_set.insert(*iter);
            }
            else
            {
                int first = iter - nums.begin();
                int sec = findIndx(nums.begin(), iter, *val);
                return std::vector<int>{first, sec}; 
            }
        }
        return std::vector<int>{-1,-1};
    }
    
    template <class InputIt>
    unsigned int findIndx(InputIt begin, InputIt end, int num)
    {
        for(InputIt iter = begin; iter!=end; ++iter)
        {
            if(num == *iter) return iter - begin;
        }
        return end - begin;
    }
    
};