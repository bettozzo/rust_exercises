use std::fmt::Display;

struct Tasks{
    tasts:Vec<Task>
}
impl Tasks{
    fn new() -> Self{
        Self { tasts: Vec::new() }
    }
}

struct Task{
    name: String,
    priority:i32,
    done:bool,
}
impl Task{
    fn new(name: String, priority:i32) -> Self{
        Self { name, priority, done: false }
    }
}
impl Display for Task{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task  **{}** has priority {}. Status: {}", self.name, self.priority, self.done)
    }
}


pub fn main_es4(){
    
}