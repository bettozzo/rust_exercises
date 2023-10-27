use std::marker::PhantomData;
use rand::Rng;

struct Gate<S>{
    _phantom: PhantomData<S>
}

struct Open;
struct Closed;
struct Stopped(String);

impl Gate<Open>{
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>>{
        let fail_chance = rand::thread_rng().gen_range(0..100);
        if fail_chance < 20{
            return Err( Gate{_phantom: PhantomData});
        }
        return Ok(Gate{_phantom: PhantomData});
    }
}
impl Gate<Closed>{
    fn open(self) -> Result<Gate<Open>, Gate<Stopped>>{
        let fail_chance = rand::thread_rng().gen_range(0..100);
        if fail_chance < 20{
            return Err(Gate{_phantom: PhantomData});
        }
        return Ok(Gate{_phantom: PhantomData});
    }
}
impl Gate<Stopped>{
    fn open(self) -> Gate<Closed>{
        return Gate { _phantom: PhantomData};
    }
    fn close(self) -> Gate<Open>{
        return Gate { _phantom: PhantomData};
    }
}

pub fn main_es6(){
    let castle_gate : Gate<Open> = Gate{ _phantom: PhantomData };
    let new_gate_state = castle_gate.close();
    match new_gate_state {
        Ok(new_gate) => {
            println!("Closed Gate");
            match new_gate.open() {
                Ok(_) => println!("Gate Re-opened"),
                Err(_) => println!("Gate stopped while re-closing it. Whoops.")
            }
        }
        Err(_) => {
            println!("Gate stopped while closing it");
        }
    }
}