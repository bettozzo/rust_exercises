use std::{cell::RefCell, fmt::Display, rc::Rc};
#[derive(Debug)]
struct Node<T: Display + PartialEq> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}
impl<T: PartialEq + Display> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next && self.prev == other.prev
    }
}
impl<T: Display + PartialEq> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

struct List<T: Display + PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}
impl<T: PartialEq + Display> PartialEq for List<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }
        for (node1, node2) in self.head.iter().zip(other.head.iter()) {
            if *node1.borrow() != *node2.borrow() {
                return false;
            }
        }
        return true;
    }
}
impl<T: Display + PartialEq> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
    fn print_list(&self) {
        match &self.head {
            Some(head) => print_list_rec(head.clone()),
            None => println!(),
        }
    }
    fn push(&mut self, element: T) {
        let mut new_node = Node {
            value: element,
            next: None,
            prev: None,
        };

        match &self.tail {
            None => {
                self.head = Some(Rc::new(RefCell::new(new_node)));
                self.tail = self.head.clone();
            }
            Some(tail) => {
                new_node.prev = Some(tail.clone());
                tail.borrow_mut().next = Some(Rc::new(RefCell::new(new_node)));
            }
        }
    }
}

pub fn main_es2() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.print_list();
}
fn print_list_rec<T: Display + PartialEq>(node: Rc<RefCell<Node<T>>>) {
    println!("{}", node.borrow());
    match &node.borrow().next {
        Some(next_node) => print_list_rec(next_node.clone()),
        None => println!(),
    }
}
