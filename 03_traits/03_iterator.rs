struct CountDown {
    i: u32
}

impl Iterator for CountDown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.i == 0 {
            return None;
        }
        self.i -= 1;
        return Some(self.i + 1);
    }
}

fn main() {
    let count_range = CountDown {i: 10};
    for i in count_range {
        println!("{}", i);
    }
}


