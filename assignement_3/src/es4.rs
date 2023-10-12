use std::{collections::HashMap, fmt::{Display, write}};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Item{
    Coke,
    Coffee,
    Soda,
    Wine
}
impl Display for Item{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self{
            Item::Coke => write!(f, "Coke"),
            Item::Coffee => write!(f, "Coffee"),
            Item::Soda => write!(f, "Soda"),
            Item::Wine => write!(f, "Wine")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Coin{
    Penny,
    TwoPennies,
    FivePennies,
    TenPennies,
    TwentyPennies,
    FiftyPennies,
    OneEuro,
    TwoEuro
}

impl Coin{
    fn to_cents(self)->u32{
        match self{
            Coin::Penny => 1,
            Coin::TwoPennies => 2,
            Coin::FivePennies => 5,
            Coin::TenPennies => 10,
            Coin::TwentyPennies => 20,
            Coin::FiftyPennies => 50,
            Coin::OneEuro => 100,
            Coin::TwoEuro => 200
        }
    }
}

#[derive(Debug)]
pub struct VendingMachine{
    pub items:HashMap<Item, usize>,
    pub coins:u32
    
}

impl VendingMachine{
    pub fn new(items: HashMap<Item, usize>) -> Self{
        VendingMachine{items, coins:0}
    }

    pub fn add_item(&mut self, item:Item, count:usize){
        match self.items.get(&item){
            Some(tot_amount) => self.items.insert(item, tot_amount+count),
            None => None
        };
    }
    pub fn insert_coin<'a>(&mut self, coin:&'a Coin) -> Result<&'a str, &'a str>{
        self.coins += coin.to_cents();
        return Ok("The amount of coins was added to the total 👍");
    }

    pub fn get_item_price(&self, item:&Item) -> u32{
        match item{
            Item::Coke => 250,
            Item::Coffee =>45,
            Item::Soda =>120,
            Item::Wine =>141
        }
    }   
    pub fn buy(&self, item:&Item) -> Result<u32, &str>{
        let cost = self.get_item_price(item);
        if self.coins < cost{
            return Err("You dont have enough coins to perform this action. 👎")
        }
        let change = self.coins-cost;
        return Ok(change);
    }
}