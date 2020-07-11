use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;
type NodePtr<T> = Option<Rc<RefCell<ListNode<T>>>>;
struct ListNode<T> {
    next: NodePtr<T>,
    pre: NodePtr<T>,
    data: T,
}
impl<T: Copy> ListNode<T> {
    fn next(&self) -> NodePtr<T> {
        match self.next.as_ref() {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }
    fn pre(&self) -> NodePtr<T> {
        match self.pre.as_ref() {
            Some(n) => Some(n.clone()),
            None => None,
        }
    }
}
struct List<T> {
    head: NodePtr<T>,
    tail: NodePtr<T>,
    count: usize,
}

impl<T: Copy + Debug + Display> List<T> {
    fn new() -> Box<List<T>> {
        Box::new(List {
            count: 0,
            head: None,
            tail: None,
        })
    }
    fn push_back(&mut self, value: T) {
        self.count += 1;
        let node = Rc::new(RefCell::new(ListNode {
            data: value,
            next: None,
            pre: None,
        }));

        if let Some(t) = self.tail.as_ref() {
            t.borrow_mut().next = Some(node.clone());
            node.borrow_mut().pre = Some(t.clone());
        } else {
            self.head = Some(node.clone());
        }
        self.tail = Some(node.clone());
    }
    fn push_front(&mut self, value: T) {
        self.count += 1;
        self.count += 1;
        let node = Rc::new(RefCell::new(ListNode {
            data: value,
            next: None,
            pre: None,
        }));

        if let Some(t) = self.head.as_ref() {
            t.borrow_mut().pre = Some(node.clone());
            node.borrow_mut().next = Some(t.clone());
        } else {
            self.tail = Some(node.clone());
        }
        self.head = Some(node.clone());
    }
    fn pop_back(&mut self) -> Option<T> {
        if let Some(t) = &self.tail.clone() {
            self.count -= 1;
            if let Some(n) = &t.borrow().pre {
                n.to_owned().borrow_mut().next = None;
                self.tail = Some(n.clone());
            } else {
                self.head = None;
                self.tail = None;
            }
            Some(t.borrow().data)
        } else {
            None
        }
    }
    fn pop_front(&mut self) -> Option<T> {
        if let Some(t) = &self.head.clone() {
            self.count -= 1;
            if let Some(n) = &t.borrow().next {
                n.to_owned().borrow_mut().pre = None;
                self.head = Some(n.clone());
            } else {
                self.head = None;
                self.tail = None;
            }
            Some(t.borrow().data)
        } else {
            None
        }
    }
    fn front(&mut self) -> NodePtr<T> {
        self.head.clone()
    }
    fn back(&mut self) -> NodePtr<T> {
        self.tail.clone()
    }
    fn print(&mut self) {
        let mut cur = self.front();
        while let Some(n) = cur.clone() {
            print!("{}", n.as_ref().borrow().data);
            cur = n.borrow().next();
            if let Some(_) = cur.clone() {
                print!(" ")
            }
        }
        print!("\n");
    }
    fn print_reverse(&mut self) {
        let mut cur = self.back();
        while let Some(n) = cur.clone() {
            print!("{}", n.as_ref().borrow().data);
            cur = n.borrow().pre();
            if let Some(_) = cur.clone() {
                print!(" ")
            }
        }
        print!("\n");
    }
}

fn main() {
    let mut list = List::<i32>::new();
    list.push_back(1);
    list.print();
    list.pop_back();
    list.print();
    list.print_reverse();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_front(7);
    println!("len:{}", list.count);
    if let Some(n) = &list.front() {
        println!("back:{}", n.borrow().data);
    }
    list.print();
    list.print_reverse();
    list.pop_front();
    list.print();
    list.print_reverse();
}
