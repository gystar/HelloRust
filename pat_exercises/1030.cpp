//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805464397627392
//测试数据：
/*
Sample Input:
4 5 0 3
0 1 1 20
1 3 2 30
0 3 4 10
0 2 2 20
2 3 1 20
Sample Output:
0 2 3 3 40
*/
#include <cstdio>
#include <list>
#include <vector>

using namespace std;

struct Edg {
    int w;
    int d;
    int cost;
};

struct Path {
    int d;
    int cost;
    int tag;  //-1 no way; 0 have way; 1 done
    list<int> p;
    Path() : tag(-1) {}
    Path(int vd, int vc, int vt) : d(vd), cost(vc), tag(vt) {}
    bool operator<(const Path& p) const {
        if (tag == -1) return false;
        return (d == p.d ? cost < p.cost : d < p.d);
    }
};

int main() {
    int N, M, S, D;
    scanf("%d %d %d %d", &N, &M, &S, &D);
    vector<vector<Edg> > Adj(N);
    for (int i = 0; i < M; ++i) {
        int u, v, d, c;
        scanf("%d %d %d %d", &u, &v, &d, &c);
        Edg e = {v, d, c};
        Adj[u].push_back(e);
        e.w = u;
        Adj[v].push_back(e);
    }
    // Dijkstra
    vector<Path> P(N, {0, 0, -1});
    for (auto& e : Adj[S]) {
        P[e.w] = {e.d, e.cost, 0};
        P[e.w].p.push_back(e.w);
    }
    P[S].tag = 1;
    printf("%d", S);
    for (int i = 0; i < N - 1; ++i) {
        Path min_p;
        int min_i = -1;
        for (int j = 0; j < N; ++j) {
            if (P[j].tag != 0) continue;
            if (min_i == -1 || P[j] < min_p) {
                min_p = P[j];
                min_i = j;
            }
        }
        if (min_i == D) {
            for (auto& i : min_p.p) printf(" %d", i);
            printf(" %d %d", min_p.d, min_p.cost);
            break;
        }
        P[min_i].tag = 1;
        for (auto& e : Adj[min_i]) {
            if (P[e.w].tag == 1) continue;
            Path np = Path(min_p.d + e.d, min_p.cost + e.cost, 0);
            np.p = min_p.p;
            np.p.push_back(e.w);
            P[e.w] = min(P[e.w], np);
        }
    }
    return 0;
}
