#![feature(linked_list_cursors)]
//BUG:第三种排序方式和c写出来的结果不同
use std::collections::LinkedList;
use std::env;

/** Our old friend print from ex17. */

// a typedef creates a fake type, in this
// case for a function pointer

/**
 * A classic bubble sort function that uses the
 * compare_cb to do the sorting.
 */

fn bubble_sort(list: &mut LinkedList<i32>, cmp: fn(i32, i32) -> i32) {
    if list.is_empty() {
        return;
    }
    let len = list.len();
    for i in 0..len - 1 {
        let mut cur = list.cursor_front_mut();
        let mut change = false;
        for _ in 0..(len - 1) {
            let current = cur.current().unwrap().clone();
            let next = cur.peek_next().unwrap().clone();
            if cmp(current, next) > 0 {
                *cur.current().unwrap() = next;
                *cur.peek_next().unwrap() = current;
                change = true;
            }
            cur.move_next();
        }
        if !change {
            break;
        }
    }
}

fn sorted_order(a: i32, b: i32) -> i32 {
    return a - b;
}

fn reverse_order(a: i32, b: i32) -> i32 {
    return b - a;
}

fn strange_order(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 {
        return 0;
    } else {
        return a % b;
    }
}

/**
 * Used to test that we are sorting things correctly
 * by doing the sort and printing it out.
 */
fn test_sorting(list: &mut LinkedList<i32>, cmp: fn(i32, i32) -> i32) {
    bubble_sort(list, cmp);
    for i in list.iter() {
        print!("{:?} ", i);
    }
    print!("\n");
}

fn main() {
    let mut args = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }
    if args.len() < 3 {
        panic!("USAGE: ex18 4 3 1 5 6");
    }

    let mut numbers = LinkedList::<i32>::new();
    for i in 1..args.len() {
        numbers.push_back((&args[i]).parse::<i32>().unwrap());
    }

    test_sorting(&mut numbers, sorted_order);
    test_sorting(&mut numbers, reverse_order);
    test_sorting(&mut numbers, strange_order);
}
