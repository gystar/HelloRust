#[cfg(test)]
mod tests {
    use std::{thread, time};
    #[test]
    //在vscode中打开断点选项即可调试
    fn test_thread_sleep() {
        let now = time::SystemTime::now();
        for i in 0..100 {
            thread::sleep(time::Duration::from_millis(100));
            println!("{}", i);
        }
        assert!(now.elapsed().unwrap() >= time::Duration::from_millis(10000));
    }
}
