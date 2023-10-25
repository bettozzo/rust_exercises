use std::fmt::{Debug, Display};

fn restricted<T:PartialOrd + Debug,U:Display>(t1:T, t2:T, u:U){
    print!("minor:\t");
    if t1 < t2{
        println!("{:?}", t1);
    }else{
        println!("{:?}", t2);
    }
    println!("u:\t{}", u);
}


pub fn main_es3(){
    restricted("ciao", "ciao_grande", "ciaoU");
    println!();
    restricted(6, 4, "Uè");
}