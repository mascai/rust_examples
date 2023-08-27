use std::thread;


fn main() {
    let handle: thread::JoinHandle<i32> = thread::Builder::new()
        .name("my_thread".to_string())
        .stack_size(32 * 1024)
        .spawn(|| {
            println!("Start thread");
            return 123;
        }).unwrap();
    let val = handle.join().unwrap();
    assert_eq!(val, 123);
    
}
