
use std::{fmt::Display, path::Iter};

#[derive(Debug)]
struct Tasks{
    tasks:Vec<Task>
}
impl Tasks{
    fn new() -> Self{
        Self { tasks: Vec::new() }
    }
}
impl Iterator for Tasks{
    type Item = Task;
    fn next(&mut self) -> Option<Self::Item> {
        return self.tasks.iter().position(|task| !task.done).map(|task| self.tasks.remove(task));
    }
}

#[derive(Debug, Clone)]
struct Task{
    name: String,
    priority:i32,
    done:bool,
}
impl Task{
    fn new(name: &str, priority:i32) -> Self{
        Self { name: name.to_string(), priority, done: false }
    }
    fn complete(&mut self){
        self.done = true;
    }
}
impl Display for Task{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Task  **{}** has priority {}. Status: {}", self.name, self.priority, self.done)
    }
}

#[allow(unused_mut)]
pub fn main_es4(){
    let mut tasks = Tasks::new();
    let mut task1 = Task::new("task1", 10);
    let mut task2 = Task::new("task2", 10);
    let mut task3 = Task::new("task3", 10);
    let mut task4 = Task::new("task4", 10);
    let mut task5 = Task::new("task5", 10);
    task1.complete();
    task3.complete();
    task5.complete();
    tasks.tasks = vec![task1, task2, task3, task4, task5];
    for ts in tasks{
        print!("{:?}", ts);
    }
    // println!("{:?}", tasks);
}