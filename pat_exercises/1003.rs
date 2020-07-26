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

#[derive(Debug, Copy, Clone)]
struct Rec {
    len: usize,
    num: usize,
}

fn small(rec: &Rec, len: usize, num: usize) -> bool {
    rec.len < len || (rec.len == len && rec.num > num)
}

impl PartialOrd for Rec {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if small(&self, other.len, other.num) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for Rec {
    fn cmp(&self, other: &Self) -> Ordering {
        if small(&self, other.len, other.num) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialEq for Rec {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len
    }
}

impl Eq for Rec {}

static mut nums: Vec<usize> = Vec::new(); //每个城市的救援者人数
static mut M: Vec<Vec<usize>> = Vec::new(); //图的邻接矩阵，-1表明没有路径
static mut visited: Vec<bool> = Vec::new();
//目的地，当前结点，当前花费，最小花费记录，所有能到达目的地的记录，当前已经在走过的路径
fn DFS(
    dest: usize,
    v: usize,
    dist: Rec,
    min_dist: &mut Rec,
    paths: &mut Vec<Rec>,
    P: &mut LinkedList<usize>,
) {
    unsafe {
        if v == dest {
            //arrived
            if dist < *min_dist {
                *min_dist = dist;
            }
            paths.push(dist);
            return;
        }

        if dist.len > min_dist.len {
            //剪枝
            return;
        }

        visited[v] = true;
        for w in 0..M[v].len() {
            if M[v][w] == usize::MAX || visited[w] {
                //如果不排除为无穷大的情况，即-1，则加法可能出问题。认为unsigned -1为无穷，则和常数的加法也应该为无穷
                continue;
            }
            P.push_back(w);
            let r = Rec {
                len: dist.len + M[v][w],
                num: dist.num + nums[w],
            };
            DFS(dest, w, r, min_dist, paths, P);
            P.pop_back();
        }
        visited[v] = false;
    }
}

fn main() {
    unsafe {
        let mut arr = Vec::<usize>::new();
        if read_num_arry(&mut arr) {
            let n = arr[0];
            let m = arr[1];
            let c1 = arr[2];
            let c2 = arr[3];

            read_num_arry(&mut nums);
            M.resize(n, Vec::new());
            for v in M.iter_mut() {
                v.resize(n, usize::MAX);
            }

            for _ in 0..m {
                let mut v1 = Vec::<usize>::new();
                read_num_arry(&mut v1);
                unsafe {
                    M[v1[0]][v1[1]] = v1[2];
                }
            }

            visited.resize(n, false);

            let mut paths = Vec::<Rec>::new(); //the rec of all the paths

            visited[c1] = true;

            let mut P = LinkedList::<usize>::new();
            P.push_back(c1);
            let r0 = Rec {
                len: 0,
                num: nums[c1],
            };

            let mut min_r = Rec {
                len: usize::MAX,
                num: 0,
            };
            DFS(c2, c1, r0, &mut min_r, &mut paths, &mut P);
            let c = paths.iter().filter(|x| (**x) == min_r).count();
            println!("{} {}", c, min_r.num);
        }
    }
}
