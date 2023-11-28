use std::{cell::RefCell, rc::Rc};

trait Sound {
    fn make_sound(&self) -> String;
}

#[derive(Clone, Debug)]
struct Animals {
    name: String,
}
impl Sound for Animals {
    fn make_sound(&self) -> String {
        format!("*{} SOUND*!", self.name)
    }
}

#[derive(Debug)]
struct FarmCell<S: Sound> {
    element: S,
    next: Option<Rc<RefCell<FarmCell<S>>>>,
}

impl FarmCell<Animals> {
    fn new(obj: Animals) -> Self {
        Self {
            element: obj,
            next: None,
        }
    }
    fn insert(&mut self, obj: &Animals) {
        // println!("{:?}", self);
        match &self.next {
            Some(next_node) => {
                next_node.borrow_mut().insert(obj);
            }
            None => {
                self.next = Some(Rc::new(RefCell::new(Self {
                    element: obj.to_owned(),
                    next: None,
                })))
            }
        }
    }
}
impl Sound for FarmCell<Animals> {
    fn make_sound(&self) -> String {
        let mut output = "".to_string();
        output.push_str(self.element.make_sound().as_str());
        output.push('\n');

        match &self.next {
            Some(next_node) => {
                output.push_str(next_node.borrow().make_sound().as_str());
            }
            None => (),
        }
        output
    }
}

pub fn main_es3() {
    let animal1: Animals = Animals {
        name: "Ethan".to_string(),
    };
    let animal2 = Animals {
        name: "Lola".to_string(),
    };
    let animal3 = Animals {
        name: "Jessie".to_string(),
    };
    let mut farm = FarmCell::new(animal1);
    farm.insert(&animal2);
    farm.insert(&animal3);

    println!("Listen to all of these sounds:\n{}", farm.make_sound());
}
