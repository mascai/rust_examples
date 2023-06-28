use std::fs;
use serde_json;


fn main() -> Result<(), serde_json::Error> {
    // read file into string
    let res = fs::read_to_string("data.json");
    let s = match res {
        Ok(s) => s,
        Err(_) => panic!("Can't read file")
    };

    // convert string to json object
    let mut json_data: serde_json::Value = serde_json::from_str(&s)
        .expect("Can't parse json");

    println!("Data: {}", json_data);

    println!("Bob age: {}", json_data[0]["age"]);

    // change values
    json_data[0]["age"] = serde_json::json!(123);
    json_data[0]["name"] = serde_json::json!("Mascai");
    println!("Data: {}", json_data);
    
    std::fs::write("output.json", serde_json::to_string_pretty(&json_data).unwrap())
        .expect("Can't write to file");


    Ok(())
}