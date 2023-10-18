use std::fmt::Display;

trait Object{
    fn build(&self) -> &'static str;
    fn get_quantity(&self) -> String;
}
struct Chair<'a>{
    color: &'a str,
    quantity: &'a usize,
}
impl <'a> Object for Chair<'a>{
    fn build(&self) -> &'static str {
        "Chair has been built!"
    }
    fn get_quantity(&self) -> String {
        format!("This many chairs: {}", self.quantity)
    }
}
impl <'a> Display for Chair<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.quantity{
            0 => {write!(f, "No chairs")},
            1 => {write!(f, "One chair!!!! And its coloured: {}", self.color)},
            2 => {write!(f, "Lucky!!! Two chairs! Coloured: {}", self.color)},
            _ => {write!(f, "Way too many chairs!!! Coloured: {}", self.color)},
        }
    }
}

struct Wardrobe<'a>{
    color: &'a str,
    quantity: &'a usize,
}
impl <'a> Object for Wardrobe<'a>{
    fn build(&self) -> &'static str {
        "Wardrobe has been built"
    }
    fn get_quantity(&self) -> String {
        format!("This many wardrobes: {}", self.quantity)
    }
}
impl <'a> Display for Wardrobe<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.quantity{
            0 => {write!(f, "No wardrobe")},
            1 => {write!(f, "One wardrobe!!!! And its coloured: {}", self.color)},
            2 => {write!(f, "Lucky!!! Two wardrobes! Coloured: {}", self.color)},
            _ => {write!(f, "Way too many wardrobes!!! Coloured: {}", self.color)},
        }
    }
}


pub fn main_es8(){
    println!("***Es 8:");

    let chair = Chair{color: "red", quantity:&1};
    println!("{}", chair);

    let wardrobe = Wardrobe{color: "red", quantity:&5};
    println!("{}", wardrobe);
}