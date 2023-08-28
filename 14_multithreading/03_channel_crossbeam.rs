use std::{thread, sync::mpsc::channel};
use crossbeam::channel;


fn main() {
    let (sender, receiver) = channel();
    let handler = thread::spawn(move || {
            let xs = receiver.recv().unwrap();
            println!("{:?}", xs);
        });
    let v = vec![1, 2, 3];
    let res = sender.send(v).unwrap();
    handler.join().unwrap();
    
}
