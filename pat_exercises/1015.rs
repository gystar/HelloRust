//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805511923286016
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

fn to_r_dec(n: isize, r: isize) -> isize {
    let mut a = n;
    let mut r_n = Vec::new();
    while a > 0 {
        let odd = a % r;
        a /= r;
        r_n.push(odd);
    }
    let mut sum = 0;
    let mut p = 1;
    for i in r_n.iter().rev() {
        sum += p * (*i);
        p *= r;
    }
    println!("{}", sum);
    sum
}

fn is_prime(a: isize) -> bool {
    if a == 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= a {
        if a % i == 0 {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    let mut arr = Vec::<isize>::new();
    while read_num_arry(&mut arr) && arr[0] >= 0 {
        if arr[0] == 0 {
            println!("No");
            continue;
        }
        let res = if is_prime(arr[0]) && is_prime(to_r_dec(arr[0], arr[1])) {
            "Yes"
        } else {
            "No"
        };
        println!("{}", res);
        arr.clear();
    }
}
