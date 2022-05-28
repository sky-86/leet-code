#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let sum = 0;

    for list in [&l1, &l2] {
        let mut num = 0;
        while let Some(l) {
            num *= 10;
            num += l.val;
        }
                
        println!("{}", num);
    }
    None
}

fn main() {}
