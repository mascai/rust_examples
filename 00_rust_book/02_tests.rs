https://doc.rust-lang.org/book/ch11-01-writing-tests.html

/* rectangle.rs

pub struct  Rectangle {
    a: f32,
    b: f32
}

impl Rectangle {
    pub fn new(a: f32, b: f32) -> Rectangle {
        Rectangle {a, b}
    }

    pub fn perimeter(&self) -> f32 {
        return self.a + self.a + self.b + self.b;
    }

    pub fn square(&self) -> f32 {
        return self.a * self.b;
    }
}






 */



mod rectangle;


#[cfg(test)]
mod tests {
    use crate::rectangle;


    #[test]
    fn it_works() {
        let r = rectangle::Rectangle::new(1f32, 2f32);
        assert_eq!(r.square(), 2.);
        assert_eq!(r.perimeter(), 6.);
    }

    #[test]
    #[should_panic]
    fn not_works() {
        panic!("TEST PANIC");
    }
}
