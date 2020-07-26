//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805523835109376
//测试数据：
/*
5 6 0 2
1 2 1 5 3
0 1 1
0 2 2
0 3 1
1 2 1
2 4 1
3 4 1
*/
#include <algorithm>
#include <iostream>
#include <list>
#include <vector>
using namespace std;

struct Rec {
    unsigned len;
    unsigned num;
    bool operator<(const Rec& r) const {
        if (len == r.len)
            return num > r.num;
        else
            return len < r.len;
    }
    bool operator==(const Rec& r) const { return (len == r.len); }
};

vector<unsigned> nums;        //每个城市的救援者人数
vector<vector<unsigned> > M;  //图的邻接矩阵，-1表明没有路径
vector<bool> visited;
//目的地，当前结点，当前花费，最小花费记录，所有能到达目的地的记录，当前已经在走过的路径
void DFS(int dest, int v, Rec dist, Rec& min_dist, vector<Rec>& paths,
         list<int>& P) {
    if (v == dest) {  // arrived
        min_dist = min(min_dist, dist);
        paths.push_back(dist);
        return;
    }

    if (dist.len > min_dist.len)  //剪枝
        return;

    visited[v] = true;
    for (size_t w = 0; w < M[v].size(); ++w) {
        if (M[v][w] == (unsigned)-1 ||
            visited
                [w])  //如果不排除为无穷大的情况，即-1，则加法可能出问题。认为unsigned
                      //-1为无穷，则和常数的加法也应该为无穷
            continue;
        P.push_back(w);
        Rec r = {dist.len + M[v][w], dist.num + nums[w]};
        DFS(dest, w, r, min_dist, paths, P);
        P.pop_back();
    }
    visited[v] = false;
}

int main() {
    int n, m, c1, c2;
    while (cin >> n >> m >> c1 >> c2) {
        nums.clear();
        M.clear();
        visited.clear();
        nums.resize(n);
        for (int i = 0; i < n; ++i) {
            cin >> nums[i];
        }
        M.resize(n);
        for (int i = 0; i < n; ++i) {
            M[i].resize(n, -1);
        }
        for (int i = 0; i < m; ++i) {
            int u, v;
            unsigned d;
            cin >> u >> v >> d;
            if (u == v) continue;
            M[u][v] = min(M[u][v], d);
            M[v][u] = M[u][v];
        }
        visited.resize(n, false);
        vector<Rec> paths;  // the rec of all the paths
        visited[c1] = true;
        list<int> P;
        P.push_back(c1);
        Rec r0 = {0, nums[c1]};
        Rec min_r = {(unsigned)-1, 0};
        DFS(c2, c1, r0, min_r, paths, P);
        int c = count(paths.begin(), paths.end(), min_r);
        cout << c << " " << min_r.num << endl;
    }

    return 0;
}
