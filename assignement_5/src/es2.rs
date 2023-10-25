use rand::{distributions::Alphanumeric, Rng, random};
trait Populatable<'a>{
    fn populate(&mut self);
}

#[derive(Debug, Default)]
enum Category{
    #[default] ARTICLE,
    MAGAZINE,
}

#[derive(Debug)]
struct Book<'a>{
    title: &'a str,
    cat: Category,
}
impl Default for Book<'_>{
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let cat = match rng.gen_range(0..1) { // rand 0.8
            0 => Category::ARTICLE,
            1 => Category::MAGAZINE,
            _ => Category::ARTICLE
        };

        //todo make random title
        Self { title: "ciao", cat }
    }
}
impl<'a> Book<'a>{
    fn default_with_cat(cat: Category) -> Self{
        let mut rng = rand::thread_rng();
        
        let mut title: String = String::new();
        let random_char = rng.gen_range('a' as u8..'z' as u8) as char;
        title.push(random_char);
        

        
        Self { title:title.as_str().clone(), cat }
    }
}

#[derive(Debug, Default)]
struct Library<'a>{
    bookcases: [Vec<Book<'a>>; 10],
}
impl<'a> Populatable<'a> for Library<'a>{
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