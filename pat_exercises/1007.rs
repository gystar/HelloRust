//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805514284679168
//测试数据：
/*
Sample Input:
10
-10 1 2 3 4 -5 -23 3 7 -21
Sample Output:
10 1 4
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

fn cmp_rec(a: &Rec, b: &Rec) -> bool {
    a.sum > b.sum
}

#[derive(Copy, Clone)]
struct Rec {
    sum: isize,
    l: usize,
    h: usize,
}

fn main() {
    let mut N = Vec::<usize>::new();
    while read_num_arry(&mut N) {
        let mut arr = Vec::<isize>::new();
        read_num_arry(&mut arr);
        let num_negative = arr.iter().filter(|x| **x < 0).count();
        if num_negative == arr.len() {
            println!("0 {} {}", arr.first().unwrap(), arr.last().unwrap());
        } else {
            let mut M = Vec::<Rec>::new();
            M.resize(N[0], Rec { sum: 0, l: 0, h: 0 });
            let (mut max_sum, mut max_l) = if arr[0] > 0 { (arr[0], 0) } else { (0, 1) };
            for i in 1..N[0] {
                max_sum += arr[i];
                let r = Rec {
                    sum: max_sum,
                    l: max_l,
                    h: i,
                };
                M[i] = if cmp_rec(&r, &M[i - 1]) { r } else { M[i - 1] };
                if max_sum < 0 {
                    max_sum = 0;
                    max_l = i + 1;
                }
            }
            println!(
                "{} {} {}",
                M[N[0] - 1].sum,
                arr[M[N[0] - 1].l],
                arr[M[N[0] - 1].h]
            );
        }
    }
}
