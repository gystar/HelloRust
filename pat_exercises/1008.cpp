//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805511923286016
//测试数据：
/*
Sample Input:
3 2 3 1
Sample Output:
41
*/
#include <iostream>
#include <vector>
using namespace std;

int main() {
    int n;
    while (cin >> n && n > 0) {
        int sum = 0;
        int pre = 0;
        int f;
        for (int i = 0; i < n; ++i) {
            cin >> f;
            sum += 5;
            if (f > pre) {
                sum += 6 * (f - pre);
            } else {
                sum += 4 * (pre - f);
            }
            pre = f;
        }
        cout << sum << endl;
    }
    return 0;
}
