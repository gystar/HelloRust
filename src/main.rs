extern crate std;

fn main() {
    println!("Hello,guiyi！");
    pub fn answer() -> () {
        let a = 1;
        let b = 2;
        assert_eq!(sum(a, b), 3);
    }
    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    answer();
}
