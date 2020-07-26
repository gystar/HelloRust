//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805521431773184
//测试数据：
/*
Sample Input:
12345
Sample Output:
one five
*/
#include <cstdio>
#include <iostream>
#include <list>
using namespace std;

static const char* keys[] = {"zero", "one", "two",   "three", "four",
                             "five", "six", "seven", "eight", "nine"};

int main() {
    char c;
    list<int>
        res;  //使用list可以方便的进行正反向顺序访问和头尾插入，是累加器的很好选择
    res.push_back(0);  //有可能只有一个0进来
    while (EOF != scanf("%c", &c) && c != '\n') {
        int odd = c - '0';  //这个变量作为进位使用
        for (list<int>::reverse_iterator it = res.rbegin();
             odd != 0 && it != res.rend(); ++it) {
            if (odd == 0) break;
            int v = (odd + *it) % 10;
            odd = (odd + *it) / 10;
            *it = v;
        }
        if (odd != 0) res.push_front(odd);
    }

    // output
    list<int>::iterator it = res.begin();
    printf("%s", keys[*(it++)]);
    while (it != res.end()) {
        printf(" %s", keys[*(it++)]);
    }

    return 0;
}
