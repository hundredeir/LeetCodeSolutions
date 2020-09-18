/*
https://leetcode.com/problems/longest-substring-without-repeating-characters/
*/

class Solution {
public:
    
    int lengthOfLongestSubstring(const string s) {
        unsigned int max_len = 0;
        unsigned int head = 0;
        std::unordered_map<char, int> pos_map;
        
        unsigned int indx = 0;
        int sz = s.size();
        while(indx != sz)
        {
            char val = s[indx];
            std::unordered_map<char, int>::iterator pair_val = pos_map.find(val);
            if(pair_val != pos_map.end() && pair_val->second >= head)
            {
                int position = pair_val->second;
                int len = (indx - head);
                if(len > max_len)
                {
                    max_len = len;
                }
                head = position+1;
            }
            pos_map[val] = indx;
            ++indx;
        }
        
        if((indx - head) > max_len)
        {
            max_len = indx - head;
        }
        return max_len;
    }
};