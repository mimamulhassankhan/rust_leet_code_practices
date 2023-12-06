// Definition for singly-linked list.

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

pub fn convert_list_to_decimal(l: Option<Box<ListNode>>) -> i32 {
    let mut result = 0;
    let mut l = l;
    let mut i = 0;
    while let Some(node) = l {
        result += node.val * 10_i32.pow(i);
        l = node.next;
        i += 1;
    }
    result
}

pub fn decimal_to_list(mut n: i32) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(n % 10)));
    let mut tail = &mut head;
    n /= 10;
    while n > 0 {
        *tail.next = Some(Box::new(ListNode::new(n % 10)));
        n /= 10;
    }
    println!("{:?}", head);
    head
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let result = convert_list_to_decimal(l1) + convert_list_to_decimal(l2);
    let list = decimal_to_list(result);
    list
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode::new(3))),
        })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode::new(4))),
        })),
    }));
    let result = add_two_numbers(l1, l2);
    println!("{:?}", result);
}
