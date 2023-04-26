fn main() {
    let water  = Some("water");
    let mut empty_glass: Option<&str> = None;

    if water.is_some() {
        println!("{}", water.unwrap());
    }

    if empty_glass.is_none() {
        println!("No water");
    }

    empty_glass = Some("WATER2");
    if empty_glass.is_some() {
        println!("WATER2");
    }
}
