//题目链接:https://pintia.cn/problem-sets/994805342720868352/problems/994805470349344768
//测试数据：
/*
Sample Input:
15 43 71
Sample Output:
#123456
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
fn i_to_marnum(n: u8) -> char {
    if n < 10 {
        (n + '0' as u8) as char
    } else {
        (n % 10 + 'A' as u8) as char
    }
}

fn main() {
    print!("#");
    let mut arr = Vec::<u8>::new();
    read_num_arry(&mut arr);
    for i in arr {
        print!("{}{}", i_to_marnum(i / 13), i_to_marnum(i % 13));
    }
}
