/*
Aim - implement custom class in Rust
*/


mod my_math {
    use std::fmt;
    use std::ops;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    /// Complex numbers (real, image * i)
    pub struct Complex {
        pub real: i32,
        pub imaginary: i32,
    }

    impl Complex {
        pub fn new(real: i32, imaginary: i32) -> Complex {
            Complex { real, imaginary }
        }
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {}i)", self.real, self.imaginary)
        }
    }

    impl ops::Add for Complex {
        type Output = Complex;
        fn add(self, rhs: Complex) -> Complex {
            Complex {
                real: rhs.real + self.real,
                imaginary: rhs.imaginary + self.imaginary,
            }
        }
    }

    impl ops::Sub for Complex {
        type Output = Complex;
        fn sub(self, rhs: Complex) -> Complex {
            Complex {
                real: self.real - rhs.real,
                imaginary: self.imaginary - rhs.imaginary,
            }
        }
    }

    impl ops::Mul for Complex {
        type Output = Complex;
        fn mul(self, rhs: Complex) -> Self::Output {
            Complex {
                real: self.real * rhs.real - self.imaginary * rhs.imaginary,
                imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
            }
        }
    }
}

fn main() {
    let c0 = my_math::Complex::default();
    assert_eq!(c0.real, 0);
    assert_eq!(c0.imaginary, 0);

    let c1 = my_math::Complex {
        real: 1,
        imaginary: 10,
    };
    let c2 = my_math::Complex::new(2, 20);

    let c3 = c1 + c2;
    let c3_expected = my_math::Complex::new(3, 30);
    assert_eq!(c3, c3_expected);

    println!("{}", c3);
}
