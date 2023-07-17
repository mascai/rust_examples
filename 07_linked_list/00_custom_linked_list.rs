

#[derive(Eq, PartialEq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>
}
impl ListNode {
    fn new(val: i32) -> Self {
        println!("Call new {}", val);
        return ListNode {
            val: val,
            next: None
        };
    }

    fn push_front(self, mut new_head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        new_head.as_mut().unwrap().next = Some(Box::new(self));
        return new_head;
    }

    fn push_back(&mut self, new_node: Option<Box<ListNode>>) {
        let mut cur = self;
        while cur.next.is_some() {
            cur = cur.next.as_mut().unwrap();
        }
        cur.next = new_node;
    }

    pub fn reverse(self) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = Some(Box::new(self));
        let mut next = None;

        while let Some(mut node) = curr {
            next = node.next;
            node.next = prev;
            prev = Some(node);
            curr = next;
        }
        return prev;
    }

    fn count(&self) -> u32 {
        match self.next {
            Option::Some(ref next) => 1 + next.count(),
            Option::None => 1, // By default ListNode object is not empty
        }
    }
}

/* TODO
fn len(&mut head: &mut Option<Box<ListNode>>) -> usize {
    let mut cur = head;
    let mut res = 0 as usize;
    while cur.is_some() {
        cur = cur.unwrap().next;
        res += 1;
    }
    return res;
}*/


impl Default for ListNode {
    fn default() -> Self {
        println!("Call default");
        return ListNode {
            val: 0,
            next: None
        };
    }
}

/* 
impl Drop for ListNode {
    fn drop(&mut self) {
        println!("Call drop {}", self.val);
        
    }
}*/


fn main() {
    let mut node1 = Some(Box::new(ListNode::new(1))); 
    let mut node2 = Some(Box::new(ListNode::new(2)));
    let mut node3 = Some(Box::new(ListNode::new(3)));

    node2 = node1.unwrap().push_front(node2);  // node2 -> node1 -> None
    node2.as_mut().unwrap().push_back(node3); // node2 -> node1 -> node3 -> None
    println!("List1 (initial): {:?}", node2);
    
    let mut reversed_list = node2.unwrap().reverse();
    println!("List2 (reversed): {:?}", reversed_list);
    println!("List len: {:?}", reversed_list.as_mut().unwrap().count());
    println!("List3 (reversed): {:?}", reversed_list);

}

