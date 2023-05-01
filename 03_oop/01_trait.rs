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
        println!("Wow")
    }
}


fn say2<T: Say>(t: &T) {
    t.say();
    t.say();
}


fn main() {
    let cat = Cat;
    say2(&cat);
}
