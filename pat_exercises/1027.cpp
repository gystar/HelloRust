//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805470349344768
//测试数据：
/*
Sample Input:
15 43 71
Sample Output:
#123456
*/

#include <cstdio>
char i_to_marnum(int n) {
    if (n < 10)
        return n + '0';
    else
        return n % 10 + 'A';
}

int main() {
    printf("#");
    for (int i = 0; i < 3; ++i) {
        int n;
        scanf("%d", &n);
        printf("%c%c", i_to_marnum(n / 13), i_to_marnum(n % 13));
    }
    return 0;
}
