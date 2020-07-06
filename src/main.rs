extern crate std;

fn foo(s: String) -> String {
    let w = "World".to_string();
    s + &w
}

fn main() {
    let s = "Hello".to_string();
    let ss = foo(s);
    println!("{}", ss);
    //println!("{}", s);     // 编译出错"borrowed of moved value"
}
