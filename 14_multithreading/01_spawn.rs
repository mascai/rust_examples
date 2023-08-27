use std::thread;


fn main() {
    let handle = thread::spawn(|| {
        println!("Start thread");
        return 123;
    });
    let val = handle.join().unwrap();
    assert_eq!(val, 123);

}
