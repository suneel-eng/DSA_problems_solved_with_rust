#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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

pub fn print_linked_list(head: Option<Box<ListNode>>) -> Vec<u8> {
    let mut items: Vec<u8> = Vec::new();

    let mut temp = head;

    while temp.is_some() {

        items.push(temp.as_ref().unwrap().data);

        temp = temp.unwrap().next;
    }

    items
}

pub fn insert_in_linked_list(head: Option<Box<ListNode>>, element: u8, position: usize) -> Option<Box<ListNode>> {

    let mut node = Option::Some(
        Box::new(
            ListNode::new(element)
        )
    );

    if position == 0 {
        if let Some(ref mut value) = node {
            value.next = head;
            return node;
        }
    }

    let mut index: usize = 0;

    let mut head = head;

    let mut temp: &mut Option<Box<ListNode>> = &mut head;

    while temp.is_some() && index < position - 1 {

        index += 1;

        if let Some(ref mut value) = temp {
            temp = &mut value.next;
        }
    }

    if let (Some(ref mut value), Some(ref mut temp_value)) = (node.as_mut(), temp.as_mut()) {
            value.next = temp_value.next.clone();
            temp_value.next = node;
    }

    return head;
}

pub fn delete_in_linked_list(head: Option<Box<ListNode>>, position: usize) -> Option<Box<ListNode>> {

    if position == 0 {
        return head.unwrap().next;
    }

    let mut index: usize = 0;

    let mut head = head;
    let mut temp: &mut Option<Box<ListNode>> = &mut head;

    while temp.is_some() && index < position - 1 {

        index += 1;

        if let Some(ref mut value) = temp {
            temp = &mut value.next;
        }
    }

    if let Some(ref mut value) = temp {
        if let Some(ref next_value) = value.next {
            value.next = next_value.next.clone();
        }
    }


    head
}

pub fn element_in_linked_list(head: Option<Box<ListNode>>, position: usize) -> u8 {

    let mut index: usize = 0;
    let mut head = head;

    let mut temp: &mut Option<Box<ListNode>> = &mut head;

    while index < position {
        index += 1;

        if let Some(ref mut value) = temp {
            temp = &mut value.next;
        }
    }

    temp.as_ref().unwrap().data
}