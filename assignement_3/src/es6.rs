use std::fmt::Display;
use std::fmt::Debug;



struct Book{
    name: String,
    code: String,
    year_of_publication: u32,
    author: String,
    publishing_company: String
}
impl Debug for Book{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}, {}, {}, {}, {}", self.name, self.code, self.year_of_publication, self.author, self.publishing_company)
    }
}
struct Article{
    name: String,
    code: String,
    year_of_publication: u32,
    orchid: String   
}
impl Debug for Article{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}, {}, {}, {}", self.name, self.code, self.year_of_publication, self.orchid)
    }
}

struct Magazine{
    name: String,
    code: String,
    year_of_publication: u32,
    number: u32,
    month: u8
}

impl Debug for Magazine{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}, {}, {}, {}, {}\n", self.name, self.code, self.year_of_publication, self.number, self.month)
    }
}
pub struct Library{
    books: Vec<Book>,
    articles: Vec<Article>,
    magazines: Vec<Magazine>
}

impl Library{
    pub fn new() -> Self{
        Library{ books: Vec::new(), articles: Vec::new(), magazines: Vec::new()}
    }

    pub fn add_book(&mut self, name: String, code: String, year_of_publication: u32, author:String, publishing_company:String){
        self.books.push(Book { name, code, year_of_publication, author, publishing_company });
    }

    pub fn add_article(&mut self, name: String, code: String, year_of_publication: u32, orchid:String){
        self.articles.push(Article { name, code, year_of_publication, orchid});
    }

    pub fn add_magazine(&mut self, name: String, code: String, year_of_publication: u32, number:u32, month:u8){
        if month > 12{
            panic!("Month was higher than 12. It must be between 1 and 12")
        }
        if month == 0{
            panic!("Month was 0. It must be between 1 and 12")
        }
        self.magazines.push(Magazine { name, code, year_of_publication, number, month});
    }
}

impl Display for Library{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Books:\n{:?}\nArticles:\n{:?}\nMagazines:\n {:?}", self.books, self.articles, self.magazines)
    }
}
