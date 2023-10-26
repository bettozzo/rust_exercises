use rand::{distributions::Alphanumeric, Rng, random};
trait Populatable{
    fn populate(&mut self);
}

#[derive(Debug, Default)]
enum Category{
    #[default] ARTICLE,
    MAGAZINE,
}

#[derive(Debug)]
struct Book{
    title: String,
    cat: Category,
}
impl Default for Book{
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let cat = match rng.gen_range(0..2) { // rand 0.8
            0 => Category::ARTICLE,
            _ => Category::MAGAZINE
        };

        let mut title: String = String::new();
        for _ in 0..10{
            let random_char = rng.gen_range('a' as u8..'z' as u8) as char;
            title.push(random_char);
        }
        Self { title, cat }
    }
}
impl Book{
    fn default_with_cat(cat: Category) -> Self{
        let mut rng = rand::thread_rng();
        
        let mut title: String = String::new();
        for _ in 0..10{
            let random_char = rng.gen_range('a' as u8..'z' as u8) as char;
            title.push(random_char);
        }

        Self { title, cat }
    }
}

#[derive(Debug, Default)]
struct Library{
    bookcases: [Vec<Book>; 10],
}
impl Populatable for Library{
    fn populate(&mut self) {
        for bookcase in self.bookcases.iter_mut(){
            for _ in 0..10{
                bookcase.push(Book::default_with_cat(Category::default()));
            }
        }
    }
}


pub fn main_es2(){
    let mut library = Library::default();
    library.populate();
    println!("{:?}", library);
}