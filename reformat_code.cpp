/*
https://leetcode.com/problems/reformat-date/
*/

#include <string>
#include <map>
#include <sstream>

using namespace std;

class Solution {
    
public:
    
    string get_date(string date_string)
    {
        string ret;
        auto iter = date_string.begin();
        int val = *iter - '0';
        int i = 0;
        while (0 <= val && val <= 9)
        {
            ret.append(1, *iter);
            ++iter;
            val = *iter - '0';
            ++i;
        }
        
        if(i == 1)
        {
            ret.insert(0, 1, '0');
        }
        
        return ret;
    }

    string reformatDate(string date) {
        std::map<string, string> month_map = {{"Jan", "01"}, {"Feb", "02"}, {"Mar", "03"}, {"Apr", "04"}, {"May", "05"}, {"Jun", "06"}, {"Jul", "07"}, {"Aug", "08"}, {"Sep", "09"}, {"Oct", "10"}, {"Nov", "11"}, {"Dec", "12"}};
        istringstream iss(date);
        
        int i = 1;
        
        string ret;
        
        do
        {
            string subs;
            iss >> subs;

            switch(i)
            {
                case 1:
                    ret.insert(0, get_date(subs));
                    break;
                case 2:
                    ret.insert(0, 1, '-');
                    ret.insert(0, month_map[subs]);
                    ret.insert(0, 1, '-');
                    break;
                case 3:
                    ret.insert(0, subs);
                    break;
            }
            
            ++i;
        }
        while(iss);
        return ret;
    }
};