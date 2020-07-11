#![feature(linked_list_cursors)]
use std::collections::LinkedList;

fn bublle_sort(list: &mut LinkedList<i32>) {
    if list.is_empty() {
        return;
    }
    let len = list.len();
    for i in 0..len - 1 {
        let mut cur = list.cursor_front_mut();
        let mut change = false;
        for _ in 0..(len - 1 - i) {
            let now = cur.current().unwrap().clone();
            let next = cur.peek_next().unwrap().clone();
            if now > next {
                *cur.current().unwrap() = next;
                *cur.peek_next().unwrap() = now;
                change = true;
            }
            cur.move_next();
        }
        if !change {
            break;
        }
    }
}

fn merge(list1: &LinkedList<i32>, list2: &LinkedList<i32>) -> LinkedList<i32> {
    if list2.is_empty() {
        return list1.clone();
    }
    if list1.is_empty() {
        return list2.clone();
    }
    let mut cur1 = list1.cursor_front();
    let mut cur2 = list2.cursor_front();
    let mut list = LinkedList::new();
    while cur1.current() != None && cur2.current() != None {
        let x = cur1.current().unwrap().clone();
        let y = cur2.current().unwrap().clone();
        if x < y {
            list.push_back(x);
            cur1.move_next();
        } else {
            list.push_back(y);
            cur2.move_next();
        }
    }
    while cur1.current() != None {
        list.push_back(cur1.current().unwrap().clone());
        cur1.move_next();
    }
    while cur2.current() != None {
        list.push_back(cur2.current().unwrap().clone());
        cur2.move_next();
    }
    return list;
}

fn merge_sort(list: &mut LinkedList<i32>) {
    if list.len() < 2 {
        return;
    }
    let mut len = 2; //当前排序的链表长度，分为两半进行merge
    while len / 2 < list.len() {
        let mut list_new = LinkedList::new();
        let mut cur = list.cursor_front();
        //从前往后依次对大小为len的子链表拆分为两个进行归并排序
        while cur.current() != None {
            let mut list1 = LinkedList::new();
            let mut list2 = LinkedList::new();
            for _ in 0..len / 2 {
                if cur.current() == None {
                    break;
                }
                list1.push_back(cur.current().unwrap().clone());
                cur.move_next();
            }
            for _ in 0..len / 2 {
                if cur.current() == None {
                    break;
                }
                list2.push_back(cur.current().unwrap().clone());
                cur.move_next();
            }

            let list3 = merge(&list1, &list2);
            for v in list3.iter() {
                list_new.push_back(*v);
            }
        }
        *list = list_new;
        len *= 2;
    }
}

fn main() {
    println!("Hello");
    let mut list = LinkedList::<i32>::new();
    list.push_back(6);
    list.push_back(5);
    list.push_back(3);
    list.push_back(4);
    list.push_back(2);
    list.push_back(1);
    merge_sort(&mut list);
    println!("list after merge_sort:\n");
    for i in list {
        println!("{}", i);
    }

    let mut list2 = LinkedList::<i32>::new();
    list2.push_back(12);
    list2.push_back(11);
    list2.push_back(10);
    list2.push_back(9);
    list2.push_back(20);
    list2.push_back(155);
    bublle_sort(&mut list2);
    println!("list2 after bublle_sort:\n");
    for i in list2 {
        println!("{}", i);
    }
}
