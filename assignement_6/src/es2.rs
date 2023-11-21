use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool,
}
impl Car {
    fn new(model: &str, year: u32, price: u32) -> Self {
        Self {
            model: model.to_string(),
            year,
            price,
            rent: false,
        }
    }
}
impl Default for Car {
    fn default() -> Self {
        Self {
            model: "AAA".to_string(),
            year: 0,
            price: 0,
            rent: false,
        }
    }
}
struct CarDealer {
    cars: Vec<Rc<RefCell<Car>>>,
}

impl CarDealer {
    fn new(cars: &[Car]) -> Self {
        let mut new_self = CarDealer { cars: Vec::new() };
        for car in cars.into_iter() {
            new_self.cars.push(Rc::new(RefCell::new(car.clone())));
        }
        return new_self;
    }
    fn add_car(&mut self, car: Car) {
        self.cars.push(Rc::new(RefCell::new(car)));
    }
    fn print_cars(&self) {
        for (i, car) in self.cars.iter().enumerate() {
            let car = car.borrow();
            println!("***CAR NUMBER {}:", i + 1);
            println!(
                "\tModel: {}\n\tYear: {}\n\tPrice: {}€\n\tRented: {}",
                car.model, car.year, car.price, car.rent
            );
        }
    }
    fn rent_user(&mut self, user: &mut User, model: String) {
        for car in self.cars.iter() {
            if car.borrow().model == model && car.borrow().rent == false {
                car.borrow_mut().rent = true;
                user.maybe_car = Some(car.clone());
                return;
            }
        }
        println!("Car not found or is already rented");
    }
    fn end_rental(&mut self, user: &mut User) {
        match &user.maybe_car {
            Some(car) => {
                car.borrow_mut().rent = false;
            }
            None => {}
        }
    }
}
struct User {
    maybe_car: Option<Rc<RefCell<Car>>>,
}
impl User {
    fn print_car(&self) {
        match &self.maybe_car {
            Some(car) => {
                println!("car: {:?}", *car.borrow())
            }
            None => println!("User has no car"),
        }
    }
}

pub fn main_es2() {
    let car1 = Car::default();
    let car2 = Car::new("ABC", 100, 100000);
    let mut cardealer = CarDealer::new(&vec![car1, car2]);
    cardealer.print_cars();

    let car3 = Car::new("DFG", 100, 100000);
    cardealer.add_car(car3);
    cardealer.print_cars();

    let mut user1 = User { maybe_car: None };
    let mut user2 = User { maybe_car: None };
    cardealer.rent_user(&mut user1, "ABC".to_string());
    cardealer.rent_user(&mut user2, "ABC".to_string());
    cardealer.rent_user(&mut user2, "DGC".to_string());
    user1.print_car();
    user2.print_car();

    cardealer.end_rental(&mut user1);
    cardealer.end_rental(&mut user2);
    cardealer.print_cars();
}
