struct MyIterator<'a, T> {
    slice: &'a [T]
}

impl<'a, T> Iterator for MyIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.slice.is_empty() {
            return None;
        }
        let element = self.slice.get(0);
        self.slice = &self.slice[1..];
        return element;
    }
}

fn main() {
    let collection = vec![1, 2, 3, 4, 5];
    let iter = MyIterator{slice: &collection[..]};
    for (idx, value) in iter.enumerate() {
        assert_eq!(*value, collection[idx]);
    }
}
