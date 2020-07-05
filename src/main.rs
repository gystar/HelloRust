extern crate std;

struct Foo(i32);
struct Bar(i32, i32);
trait Inst {
    fn new(i: i32) -> Self;
}
impl Inst for Foo {
    fn new(i: i32) -> Self {
        Foo(i)
    }
}
impl Inst for Bar {
    fn new(i: i32) -> Self {
        Bar(i, i + 1)
    }
}
fn foorbar<T: Inst>(i: i32) -> T {
    T::new(i)
}

fn main() {
    let str = "Hello Rust!";
    println!("Addr:{:p},len:{}", str.as_ptr(), str.len());
    let mut arr2: [u32; 5] = [1, 2, 3, 4, 5]; //[T,len]为固定大小类型
    fn reset(arr: &mut [u32; 5]) -> () {
        arr[0] = 10;
    }
    reset(&mut arr2);
    println!("{}", arr2[0]);

    let x = "1";
    let int_x: i32 = x.parse().unwrap();
    assert_eq!(int_x, 1);

    let foo: Foo = foorbar(1);
    let bar: Bar = foorbar(2);
}
