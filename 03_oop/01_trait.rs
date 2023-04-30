// This is the main function

struct Cat;
struct Dog;


trait Say {
    fn say(&self);
}


impl Say for Cat {
    fn say(&self) {
        println!("Meow");
    }
}

impl Say for Dog {
    fn say(&self) {
        println!("Wow");
    }
}

fn main() {
    let cat = Cat;
    cat.say();
    
    let dog = Dog;
    dog.say();
}
