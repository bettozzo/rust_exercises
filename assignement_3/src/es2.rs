use std::fmt;
use std::fmt::Formatter;

pub enum _Fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

pub enum IPAdd {
    IPv4((u8,u8,u8, u8)),
    IPv6((AddressGroup,AddressGroup,AddressGroup,AddressGroup,AddressGroup,AddressGroup,AddressGroup,AddressGroup))
}

pub struct AddressGroup {
    c1: u8,
    c2: u8,
    c3: u8,
    c4: u8,
}

impl AddressGroup {
    pub fn new (c1 :u8, c2 :u8, c3:u8, c4:u8) -> Self {
        if c1 > 15 || c2 > 15 || c3 > 15 || c4 > 15 { panic!("Values must be between 0 and 15")}
        AddressGroup {
            c1,
            c2,
            c3,
            c4
        }
    }
}
impl fmt::Display for AddressGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}{:x}{:x}{:x}", self.c1, self.c2, self.c3, self.c4)
    }
}

impl fmt::Display for IPAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {

        match self {
            IPAdd::IPv4(c) => write!(f, "{}.{}.{}.{}", c.0, c.1, c.2, c.3),
            IPAdd::IPv6(c) =>  write!(f,"{}:{}:{}:{}:{}:{}:{}:{}", c.0, c.1, c.2, c.3, c.4, c.5, c.6, c.7)
        }

    }
}