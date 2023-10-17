#[derive(Debug, PartialEq)]
struct Person<'a>{
    name:String,
    father:Option<&'a Person<'a>>,
    mother:Option<&'a Person<'a>>,
}

impl <'a> Person<'a>{
    fn new(name:&str, father:Option<&'a Person>, mother: Option<&'a Person>) -> Self{
        Person{name: name.to_string(), father, mother}
    }

    fn find_relatives(&self, generations: u32) -> Vec<&'a Person>{
        let mut relatives = vec![self];
        if generations > 0{
            match self.father{
                Some(father) => relatives.push(father),
                None => {}
            };
            match self.mother{
                Some(mother) => relatives.push(mother),
                None => {}
            }
            relatives.append(&mut self.find_relatives(generations-1));
        }
        return relatives;
    }
    
    fn find_roots(&self) -> Vec<&'a Person>{
        let mut roots = Vec::new();
        match self.father{
            Some(father) => roots.append(&mut father.find_roots()),
            None => roots.push(self)
        };
        match self.mother{
            Some(mother) => roots.append(&mut mother.find_roots()),
            None => {
                if !roots.contains(&self){
                    roots.push(self);
                }
            }
        }
        return roots;
    }
}

fn find_parents<'a>(person: &'a Person) -> (Option<&'a Person<'a>>, Option<&'a Person<'a>>){
    let mut parents = (None, None);
    match person.father{
        Some(father) => parents.0 = Some(father),
        None => {}
    };
    match person.mother{
        Some(mother) => parents.1 = Some(mother),
        None => {}
    }
    return parents;
}

pub fn main_es2() {
    let nonno_mamma = Person::new("Severino", None, None);
    let nonna_mamma = Person::new("Augusta", None, None);
    let nonno_papà = Person::new("Marco", None, None);
    let nonna_papà = Person::new("Maddalena", None, None);
    let papà = Person::new("Massimo", Some(&nonna_papà), Some(&nonno_papà));
    let mamma = Person::new("Bruna", Some(&nonna_mamma), Some(&nonno_mamma));
    let io = Person::new("Federico", Some(&papà), Some(&mamma));
    // println!("{:?}",(nonno_mamma.find_relatives(3)));
    // println!("{:?}",(io.find_relatives(3)));
    println!("{:?}",(io.find_roots()));
}