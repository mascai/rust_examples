fn f<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 { // lifetime of the result is similar to x
    return x;
}

fn main() {
    let x: i32 = 10;
    let r: &i32;

    {
        let _y = 1;
        r = f(&x, &_y); 
    } 
    println!("XXX_  {}", r);
}
