trait Printable{
    fn print(&self);
}

impl Printable for i32{
    fn print(&self) {
        println!("{}", self);
    }
}

impl Printable for String{
    fn print(&self) {
        println!("{}", self);
    }
}


// ! dynamic dyspatch
// impl Printable for Vec<&dyn Printable>{
//     fn print(&self) {
//         for el in self{
//             el.print();
//         }
//     }
// }

// ! monomorhization
impl <T:Printable> Printable for Vec<T>{
    fn print(&self) {
        for el in self{
            el.print();
        }
    }
}
pub fn main_es1(){
    let number = 1;
    number.print();
    let string = String::from("ciao");
    string.print();
    // ! dynamic dyspatch
    // let vec:Vec<&dyn Printable> = vec![&4, &5, &10];

    // ! monomorphization
    let vec = vec![4, 5, 10];

    vec.print();
}