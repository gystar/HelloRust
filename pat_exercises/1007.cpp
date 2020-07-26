//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805514284679168
//测试数据：
/*
Sample Input:
10
-10 1 2 3 4 -5 -23 3 7 -21
Sample Output:
10 1 4
*/
#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

bool cmp(int a) { return a < 0; }

struct Rec {
    int sum;
    int l, h;
    bool operator<(const Rec& a) const { return sum < a.sum; }
};

int main() {
    int n;
    while (cin >> n && n > 0) {
        vector<int> arr(n);
        for (int i = 0; i < n; ++i) {
            cin >> arr[i];
        }
        size_t num_negative = count_if(arr.begin(), arr.end(), cmp);
        if (num_negative == arr.size()) {
            cout << 0 << " " << arr.front() << " " << arr.back() << endl;
        } else {
            vector<Rec> M(n);
            M[0].sum = arr[0];
            M[0].l = 0;
            M[0].h = 0;
            int max_sum = (arr[0] > 0 ? arr[0] : 0);
            int max_l = (arr[0] > 0 ? 0 : 1);
            for (int i = 1; i < n; ++i) {
                max_sum += arr[i];
                Rec r = {max_sum, max_l, i};
                M[i] = max(M[i - 1], r);
                if (max_sum < 0) {
                    max_sum = 0;
                    max_l = i + 1;
                }
            }
            cout << M[n - 1].sum << " " << arr[M[n - 1].l] << " "
                 << arr[M[n - 1].h] << endl;
        }
    }
    return 0;
}
