use std::fmt::{Display, Debug};

pub struct Date (pub u8, pub u8, pub u16);

pub struct Hour (pub u8, pub u8);

pub struct BoxShipping{
    pub name: String,
    pub barcode: String,
    pub shipment_date: Date,
    pub shipment_hour: Hour
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.0, self.1, self.2)
    }
}

impl Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

impl Display for BoxShipping{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}, {}", self.name, self.barcode, self.shipment_date, self.shipment_hour)
    }
}

impl Debug for BoxShipping{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, barcode: {}, shipment_date: {}, shipment_hour: {}", self.name, self.barcode, self.shipment_date, self.shipment_hour)
    }
}