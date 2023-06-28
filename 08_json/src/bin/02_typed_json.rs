use serde_derive::{Deserialize, Serialize};
use std::fs;


#[derive(Deserialize, Serialize, Debug)]
struct Person {
    name: String,
    gender: String,
    age: i32,

    #[serde(default)] // mark as optional field
    cars: Vec<Car>
}

#[derive(Deserialize, Serialize, Debug)]
struct Car {
    id: i32,
    name: String
}


fn main() {
    
    // serialize data into struct
    let mut people = {
        let res = fs::read_to_string("data.json").unwrap();
        serde_json::from_str::<Vec<Person>>(&res).unwrap()
    };
    
    // change data
    people[1].name = "Alexa".to_string();
    println!("{:?}", people);
    
    // save result
    fs::write("output2.json", serde_json::to_string_pretty(&people).unwrap())
        .expect("Can't write to file");
}

