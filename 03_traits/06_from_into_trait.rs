use std::convert::From;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}

struct Soul {
    name: String
}

impl From<i32> for Person {
    fn from(value: i32) -> Self {
        Person { name: "DefaultName".to_string(), age: value as u32}
    }
}

impl From<Soul> for Person {
    fn from(soul: Soul) -> Self {
        Person { name: soul.name, age: 0 }
    }
}

impl Into<i32> for Person {
    fn into(self) -> i32 {
        return self.age as i32;
    }
}

fn main() {
    let p = Person::from(12);
    println!("{:?}", p);

    let soul = Soul {name: "Lion".to_string()};
    let p = Person::from(soul);
    println!("{:?}", p);

    let age: i32 = p.into();
    println!("{}", age);
}
