/*
https://leetcode.com/problems/reverse-integer/
*/
#include <limits>


class Solution {
    std::numeric_limits<int> limit;
public:
    long int reverse(int x) {
        long int ret = 0;
        
        while(x != 0)
        {
            ret = ret * 10 + (x%10);
            x = (int)x/10;
        }
        
        if(ret > limit.max() || ret < limit.min()) return 0;
        else return ret;
    }
};