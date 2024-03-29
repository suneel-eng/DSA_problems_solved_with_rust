#[derive(Debug)]
#[derive(PartialEq)]
pub struct ListNode {
    pub data: u8,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(value: u8) -> Self {
        ListNode {
            data: value,
            next: Option::None
        }
    }
}

pub fn vector_to_linked_list(items: Vec<u8>) -> Option<Box<ListNode>> {

    let mut head = Option::Some(
        Box::new(
            ListNode::new(items[0])
        )
    );

    let mut temp = &mut head;

    for index in 1..items.len() {

        let new_node = Option::Some(
            Box::new(
                ListNode::new(items[index])
            )
        );

        if let Some(node_box) = temp {
            node_box.next = new_node;
            temp = &mut node_box.next;
        }

    }

    head
}