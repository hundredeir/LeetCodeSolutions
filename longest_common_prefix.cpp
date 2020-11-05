/*
https://leetcode.com/problems/longest-common-prefix/
*/
#include <string>
#include <unordered_set>

using namespace std;

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if(strs.size() == 0) return "";
        int i = 0;
        string ret("");
        unordered_set<string::size_type> i_map;
        for(string s : strs)
        {
            i_map.insert(s.size());
        }
        while(i_map.find(i)==i_map.end())
        {
            for(vector<string>::iterator iter = strs.begin() + 1; iter!=strs.end(); ++iter)
            {
                if(strs[0][i] != (*iter)[i])
                {
                    return ret;
                }
            }
            ret+=strs[0][i];
            ++i;
        }
        return ret;
    }
};