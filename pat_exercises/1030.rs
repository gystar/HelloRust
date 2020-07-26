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

use core::cmp::Ordering;
use std::cmp::{max, min};
use std::collections::LinkedList;
use std::io;
use std::str::FromStr;
fn read_num_arry<T: FromStr + std::fmt::Debug>(vec: &mut Vec<T>) -> bool {
    let mut buff = String::new();
    if let Err(_) = io::stdin().read_line(&mut buff) {
        return false;
    }
    let arr: Vec<&str> = buff.trim().split(" ").collect();
    for s in arr {
        if let Ok(value) = s.trim().parse::<T>() {
            vec.push(value);
        }
    }
    vec.len() > 0
}

#[derive(Copy, Clone, Default)]
struct Edg {
    w: usize,
    d: usize,
    cost: usize,
}

#[derive(Clone, Default)]
struct Path {
    d: usize,
    cost: usize,
    tag: isize, //-1 no way; 0 have way; 1 done
    p: LinkedList<usize>,
}
impl Path {
    pub fn new(d: usize, cost: usize, tag: isize) -> Self {
        Self {
            d,
            cost,
            tag,
            p: LinkedList::<usize>::new(),
        }
    }
}

fn cmp_path(p1: &Path, p2: &Path) -> bool {
    if p1.tag == -1 {
        return false;
    }
    if p1.d == p2.d {
        p1.cost < p2.cost
    } else {
        p1.d < p2.d
    }
}

fn main() {
    let mut arr1 = Vec::<usize>::new();
    read_num_arry(&mut arr1);
    let (N, M, S, D) = (arr1[0], arr1[1], arr1[2], arr1[3]);
    let mut Adj = Vec::<Vec<Edg>>::new();
    Adj.resize(N, Vec::new());
    for _ in 0..M {
        let mut arr2 = Vec::<usize>::new();
        read_num_arry(&mut arr2);
        let mut e = Edg {
            w: arr2[1],
            d: arr2[2],
            cost: arr2[3],
        };
        Adj[arr2[0]].push(e);
        e.w = arr2[0];
        Adj[arr2[1]].push(e);
    }
    // Dijkstra
    let mut P = Vec::<Path>::new();
    P.resize(N, Path::new(0, 0, -1));
    for e in Adj[S].iter() {
        P[e.w] = Path::new(e.d, e.cost, 0);
        P[e.w].p.push_back(e.w);
    }
    P[S].tag = 1;
    print!("{}", S);
    for i in 0..N - 1 {
        let mut min_p = Path::new(0, 0, -1);
        let mut min_i = usize::MAX;
        for j in 0..N {
            if P[j].tag != 0 {
                continue;
            }
            if min_i == usize::MAX || cmp_path(&P[j], &min_p) {
                min_p = P[j].clone();
                min_i = j;
            }
        }
        if min_i == D {
            for i in min_p.p {
                print!(" {}", i);
            }
            print!(" {} {}", min_p.d, min_p.cost);
            break;
        }
        P[min_i].tag = 1;
        for e in Adj[min_i].iter() {
            if P[e.w].tag == 1 {
                continue;
            }
            let mut np = Path::new(min_p.d + e.d, min_p.cost + e.cost, 0);
            np.p = min_p.p.clone();
            np.p.push_back(e.w);
            if cmp_path(&np, &P[e.w]) {
                P[e.w] = np;
            }
        }
    }
}
