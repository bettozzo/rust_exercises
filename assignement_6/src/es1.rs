use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct TreeNode<T: PartialOrd + Copy> {
    value: T,
    left: Option<Rc<RefCell<TreeNode<T>>>>,
    right: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T: PartialOrd + Copy> TreeNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
    fn from_vec(vec: &[T]) -> Self {
        let mut root = TreeNode::new(vec[0]);
        for el in vec.iter().skip(1) {
            root.insert(*el);
        }
        root
    }
    fn insert(&mut self, value: T) {
        if self.value < value {
            //go right
            match &self.right {
                Some(tree) => {
                    tree.borrow_mut().insert(value);
                }
                None => self.right = Some(Rc::new(RefCell::new(TreeNode::new(value)))),
            }
        } else {
            //go left
            match &self.left {
                Some(tree) => {
                    tree.borrow_mut().insert(value);
                }
                None => self.left = Some(Rc::new(RefCell::new(TreeNode::new(value)))),
            }
        }
    }
}

pub fn main_es1() {
    let root = TreeNode::new(5);
    println!("{:?}", root);

    let tree = TreeNode::from_vec(&[4, 6, 1, 67, 92, 4, 1]);
    println!("{:?}", tree);
}
