use std::collections::{LinkedList, linked_list};

trait SplitCustom<'a, ReturnType:'a>{
    fn split_trait(&'a self) -> (ReturnType, ReturnType);
}


impl <'a> SplitCustom<'a, &'a str> for String{
    fn split_trait(&self) -> (&str, &str) {
        self.split_at(self.len()/2)
    }
}

impl <'a> SplitCustom <'a, &'a[i32]> for &[i32]{
    fn split_trait(&'a self) -> (&'a[i32], &'a[i32]) {
        self.split_at(self.len()/2)
    }
}

impl <'a> SplitCustom <'a, LinkedList<f64>> for LinkedList<f64>{
    fn split_trait(&'a self) -> (LinkedList<f64>, LinkedList<f64>) {
        let mut first_half:LinkedList<f64> = LinkedList::new();
        let mut second_half:LinkedList<f64> = LinkedList::new();
        for (i, node) in self.iter().enumerate(){
            if i < self.len() / 2{
                first_half.push_back(*node);
            }else{
                second_half.push_back(*node);
            }
        }
        return (first_half, second_half);
    }
}




pub fn main_es5(){
    let string = "0123456789".to_string();
    println!("{:?}" , string.split_trait());

    let slice = &[0,1,2,3,4,5,6,7,8,9][..];
    println!("{:?}", slice.split_trait());
    
    let mut linked_list:LinkedList<f64> = LinkedList::new();
    linked_list.push_back(0.0);
    linked_list.push_back(1.0);
    linked_list.push_back(2.0);
    linked_list.push_back(3.0);
    linked_list.push_back(4.0);
    linked_list.push_back(5.0);
    linked_list.push_back(6.0);
    linked_list.push_back(6.0);
    linked_list.push_back(8.0);
    linked_list.push_back(9.0);
    println!("{:?}", linked_list.split_trait());
    
}