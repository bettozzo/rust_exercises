use std::collections::binary_heap;

struct BinIter {
    values: Vec<bool>,
    lenght: usize,
    counter: usize,
}
impl BinIter {
    fn new(n: u8, l: u8) -> Self {
        let mut values = Vec::new();
        for bin in format!("{:b}", n).chars() {
            if bin == '1' {
                values.push(true);
            } else {
                values.push(false);
            }
        }
        Self {
            values,
            lenght: l as usize,
            counter: 0,
        }
    }
}
impl Iterator for BinIter {
    type Item = bool;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < self.lenght {
            self.counter += 1;
            return Some(self.values[self.lenght - self.counter]);
        }
        return None;
    }
}

pub fn main_es1() {
    for n in BinIter::new(204, 8) {
        print!("{}", if n { 1 } else { 0 });
    }
    println!();
}
