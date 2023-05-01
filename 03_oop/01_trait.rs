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


fn say_twice<T: Say>(t: &T) {
    t.say();
    t.say();
}


fn say_triple<T: Say>(t: &T) {
    t.say();
    say_twice(t)
}

fn main() {
    let cat = Cat;
    say_triple(&cat);
}
