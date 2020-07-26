//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805511923286016
//测试数据：
/*
Sample Input:
3 2 3 1
Sample Output:
41
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

fn main() {
    let mut arr = Vec::<usize>::new();
    if read_num_arry(&mut arr) {
        let mut sum = 0;
        let mut pre = 0;
        for i in 1..arr.len() {
            sum += 5;
            if arr[i] > pre {
                sum += 6 * (arr[i] - pre);
            } else {
                sum += 4 * (pre - arr[i]);
            }
            pre = arr[i];
        }
        println!("{}", sum);
    }
}
