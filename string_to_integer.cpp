/*
https://leetcode.com/problems/string-to-integer-atoi/
*/
#include <string>

using namespace std;

class Solution {
public:
    int myAtoi(string str) {
        bool non_space = 0;
        int parity = 1; 
        long int ret = 0;
        
        int indx = 0;
        
        while(str[indx] == ' ') ++indx;
        if(str[indx] == '-'){
            parity = -1;   
            indx++;
        } 
        else if(str[indx] == '+'){
            parity = 1;   
            indx++;
        } 
        if(str[indx] < '0' ||  str[indx] > '9') return 0;
        
        for(auto iter = str.begin() + indx; iter!=str.end(); ++iter)
        {
            char ch = *iter;
            if(ret > INT_MAX || ret < INT_MIN) break;
            else if(ch >= '0' &&  ch <= '9')
            {
                ret = ret * 10 + (ch - '0');
            }
            else break;
        }
        
        switch(parity)
        {
            case 1:
                return (ret > INT_MAX ? INT_MAX : ret);
                break;
            case -1:
                return (ret * parity < INT_MIN ? INT_MIN : ret * parity);
                break;
        }
        return 0;
    }
};