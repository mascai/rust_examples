extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    let x = unsafe { add(62, 30)};
    println!("{}", x); // 92
}

//  rustc -l foo -L . main.rs
// env LD_LIBRARY_PATH="$LD_LIBRARY_PATH:." ./main
