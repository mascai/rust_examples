#[derive(Debug)]
struct A {
    id: i32,
    name: String
}


impl PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id && self.name == other.name;
    }
}

impl PartialOrd for A {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Also use can use return self.id.cmp(&other.id);
        if self.id > other.id {
            return Some(Ordering::Greater);
        }
        if self.id < other.id {
            return Some(Ordering::Less);
        }
        return Some(Ordering::Equal);
    }
}

fn main() {
    let a1 = A {id: 1, name: "aaaa".to_string()};
    let a2 = A {id: 1, name: "aaaa".to_string()};
    let a3 = A {id: 3, name: "aaaa".to_string()};
    assert_eq!(a1, a2);
    assert!(a1 == a2);

    assert_ne!(a1, a3);
    assert!(a1 < a3);
}
