use std::ops;
#[derive(Debug)]
struct Pair(i32, String);

impl ops::Add<i32> for Pair{
    type Output = Pair;
    fn add(self, rhs: i32) -> Self::Output {
        Pair(self.0 +rhs, self.1)
    }
}
impl ops::Add<&str> for Pair{
    type Output = Pair;
    fn add(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1 + &rhs)
    }
}

impl ops::Sub<i32> for Pair{
    type Output = Pair;
    fn sub(self, rhs: i32) -> Self::Output {
        Pair(self.0 -   rhs, self.1)
    }
}
impl ops::Sub<&str> for Pair{
    type Output = Pair;
    fn sub(self, rhs: &str) -> Self::Output {
        Pair(self.0, self.1.replace(rhs, ""))
    }
}

impl ops::Add for Pair{
    type Output = Pair;
    fn add(self, rhs: Self) -> Self::Output {
        self+rhs.0+rhs.1.as_str()
    }
}
impl ops::Sub for Pair{
    type Output = Pair;
    fn sub(self, rhs: Self) -> Self::Output {
        self-rhs.0-rhs.1.as_str()
    }
}

impl ops::Mul<i32> for Pair{
    type Output = Pair;
    fn mul(self, rhs: i32) -> Self::Output {   
        Pair(((self.0 as f32).powf(rhs as f32)) as i32, self.1.repeat(rhs as usize))
    }
}

pub fn main_es5(){
    let pair = Pair(2, "C".to_string());
    let muted_pair = pair * 5;
    println!("{:?}", muted_pair);
}