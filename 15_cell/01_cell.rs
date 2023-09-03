use std::cell::Cell;

#[derive(Debug)]
struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}


fn main() {
    let my_struct = SomeStruct {
        regular_field: 0,
        special_field: Cell::new(1),
    };
    my_struct.special_field.set(100); // we can change immutable object
    println!("{:?}", my_struct);
    
}

