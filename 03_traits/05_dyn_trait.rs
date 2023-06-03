/*
Dynamic dispatching example in rust https://doc.rust-lang.org/rust-by-example/trait/dyn.html

*/

struct Cat {}
struct Dog {}

trait Animal {
    fn say(&self);
}

impl Animal for Cat {
    fn say(&self) {
        println!("Meow");
    }
}

impl Animal for Dog {
    fn say(&self) {
        println!("Wow");
    }
}

fn process_animal(animal: &Box<dyn Animal>) {
    animal.as_ref().say();
}

fn main() {
    let cat = Cat {};
    let dog = Dog {};
    let mut v: Vec<Box<dyn Animal>> = Vec::new();
    v.push(Box::new(cat));
    v.push(Box::new(dog));
    for animal in v.iter() {
        animal.as_ref().say();
    }
    process_animal(&v[0]);
}
