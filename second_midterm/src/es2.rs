use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};
#[derive(Debug)]
struct Node<T: Display + PartialEq + Debug + Clone + Copy> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}
impl<T: PartialEq + Display + Debug + Clone + Copy> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next == other.next && self.prev == other.prev
    }
}
impl<T: Display + PartialEq + Debug + Clone + Copy> Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
struct List<T: Display + PartialEq + Debug + Clone + Copy> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}
impl<T: PartialEq + Display + Debug + Clone + Copy> PartialEq for List<T> {
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
impl<T: Display + PartialEq + Debug + Clone + Copy> List<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
    fn print_list(&self) {
        match &self.head {
            Some(head) => print_list_rec(head),
            None => println!(),
        }
    }
    fn push(&mut self, element: T) {
        let mut new_node = Node {
            value: element,
            next: None,
            prev: None,
        };

        match &self.head {
            None => {
                let pointer_to_node = &Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(pointer_to_node));
                self.tail = Some(Rc::clone(pointer_to_node));
            }
            Some(head) => {
                new_node.next = Some(Rc::clone(head));
                let pointer_to_node = &Rc::new(RefCell::new(new_node));
                head.borrow_mut().prev = Some(Rc::clone(pointer_to_node));
                self.head = Some(Rc::clone(pointer_to_node));
            }
        }
    }
    fn push_back(&mut self, element: T) {
        let mut new_node = Node {
            value: element,
            next: None,
            prev: None,
        };

        match &self.tail {
            None => {
                let pointer_to_node = &Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(pointer_to_node));
                self.tail = Some(Rc::clone(pointer_to_node));
            }
            Some(tail) => {
                new_node.prev = Some(Rc::clone(tail));
                let pointer_to_node = &Rc::new(RefCell::new(new_node));
                tail.borrow_mut().next = Some(Rc::clone(pointer_to_node));
                self.tail = Some(Rc::clone(pointer_to_node));
            }
        }
    }
    fn pop(&mut self) -> Option<T>{
        let ret_val:Option<T>;
        let next_node: Option<Rc<RefCell<Node<T>>>>;

        match &self.head{
            None => {next_node = None;ret_val=None; },
            Some(to_be_popped) => {
                match &to_be_popped.borrow().next{
                    None => next_node = None,
                    Some(next) => next_node = Some(next.clone()),
                }
                ret_val = Some(to_be_popped.borrow().value);
            },
        }
        self.head = next_node;
        return ret_val;
    }
    fn pop_back(&mut self) -> Option<T>{
        let ret_val:Option<T>;
        let prev_node: Option<Rc<RefCell<Node<T>>>>;

        match &self.tail{
            None => {prev_node = None; ret_val=None; },
            Some(to_be_popped) => {
                match &to_be_popped.borrow().prev{
                    None => prev_node = None,
                    Some(prev) => prev_node = Some(prev.clone()),
                }                
                ret_val = Some(to_be_popped.borrow().value);
            },
        }

        if ret_val.is_some(){
            self.tail.as_ref().unwrap().borrow_mut().next = None;
        }
        self.tail = prev_node;
        return ret_val;
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
    
    debug_assert_eq!(list.pop(), Some(5));
    debug_assert_eq!(list.pop(), Some(4));
    list.print_list();

    list.push_back(0);
    list.push_back(-1);
    list.print_list();


    debug_assert_eq!(list.pop_back(), Some(-1));
    debug_assert_eq!(list.pop_back(), Some(0));
    println!("after POP");
    list.print_list();
}

fn print_list_rec<T: Display + PartialEq + Debug + Clone + Copy>(node: &Rc<RefCell<Node<T>>>) {
    println!("{}", node.borrow());
    match &node.borrow().next {
        Some(next_node) => print_list_rec(next_node),
        None => println!(),
    }
}
