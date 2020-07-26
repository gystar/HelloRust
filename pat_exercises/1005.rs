//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805521431773184
//测试数据：
/*
Sample Input:
12345
Sample Output:
one five
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
fn read_string(s: &mut String) -> bool {
    if let Err(_) = io::stdin().read_line(s) {
        return false;
    }
    s.trim();
    true
}

const keys: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let mut num = String::new();
    let mut res = LinkedList::<u8>::new(); //使用list可以方便的进行正反向顺序访问和头尾插入，是累加器的很好选择
    res.push_back(0); //有可能只有一个0进来
    read_string(&mut num);
    for c in num.bytes() {
        if c < ('0' as u8) {
            continue;
        }
        let mut odd = c - ('0' as u8); //这个变量作为进位使用
        for i in res.iter_mut().rev() {
            if odd == 0 {
                break;
            }
            let v = (odd + *i) % 10;
            odd = (odd + *i) / 10;
            *i = v;
        }
        if odd != 0 {
            res.push_front(odd);
        }
    }

    for i in res.iter().take(res.len() - 1) {
        print!("{} ", keys[*i as usize]);
    }
    print!("{}", keys[*res.back().unwrap() as usize]);
}
