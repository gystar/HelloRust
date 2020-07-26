//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805495863296000
//测试数据：
/*
Sample Input:
73 10
23 2
23 10
-2
Sample Output:
Yes
Yes
No
*/

#include <math.h>

#include <iostream>
#include <stack>
using namespace std;

int to_r_dec(int n, int r) {
    int a = n;
    stack<int> r_n;
    for (int i = 0; a > 0; ++i) {
        int odd = a % r;
        a /= r;
        r_n.push(odd);
    }
    int sum = 0;
    int p = 1;
    while (!r_n.empty()) {
        sum += p * r_n.top();
        r_n.pop();
        p *= r;
    }
    return sum;
}

bool IsPrime(int a) {
    if (a == 1) return false;
    if (a == 2) return true;
    for (int i = 2; i <= ceil(sqrt(a)); ++i) {
        if (a % i == 0) {
            return false;
        }
    }
    return true;
}

int main() {
    int n, r;
    while (cin >> n && n >= 0) {
        cin >> r;
        if (n == 0) {
            cout << "No" << endl;
            continue;
        }
        bool ok1 = IsPrime(n);
        bool ok2 = IsPrime(to_r_dec(n, r));
        cout << ((ok1 && ok2) ? "Yes" : "No") << endl;
    }
    return 0;
}
