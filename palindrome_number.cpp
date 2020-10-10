/*
https://leetcode.com/problems/palindrome-number/
*/
class Solution {
public:
    bool isPalindrome(int x) {
        int temp = x;
        long int calc = 0;
        if(x < 0) return false;
        else if(x == 0) return true;
        else
        {
            while(temp!=0)
            {
                calc = calc*10 + (int)temp%10;
                temp = temp/10;
            }
            if (calc == x) return true;
            else return false;
        }
    }
};