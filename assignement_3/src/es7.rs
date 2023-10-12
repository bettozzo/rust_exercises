pub mod point{
    pub struct Point{
        pub x:f32,
        pub y:f32
    }

    impl Point{
        pub fn new(x:f32, y:f32)->Self{
            Point{x,y}
        }
        pub fn distance(&self, point:Point) -> f32{
            ((self.x-point.x).powf(2.0) + (self.y-point.y).powf(2.0)).abs().sqrt()
        }
    }
}

pub mod line{
    use super::point::Point;

    pub struct Line{
        pub start:Point,
        pub end:Point,
        m:f32,
        q:f32
    }

    impl Line{
        pub fn new(first_point:Point, second_point:Point)->Self{
            let m = calculte_m(&first_point, &second_point);
            let q = calculte_q(&first_point, &second_point);
            return Line { start: first_point, end: second_point, m, q};
        }

        pub fn contains(&self, p:&Point) -> Result<bool, String>{
            if p.y == self.m*p.x+self.q{
                return Ok(true);
            }
            return Err(String::from("Point isn't contained in the line"));
        }
    }

    fn calculte_m(first_point:&Point, second_point:&Point) -> f32{
        (first_point.y-second_point.y)/(first_point.x-second_point.x)
    }
    
    fn calculte_q(first_point:&Point, second_point:&Point) -> f32{
        (first_point.x*second_point.y-first_point.y*second_point.x)/(first_point.x-second_point.x)
    }
}

pub mod test{
    use super::line::Line;
    use super::point::Point;

    pub fn test() -> bool{
        let line=Line::new(Point::new(5.0, 5.0), Point::new(10.0, 10.0));
        let point = Point::new(6.0,3.0);
        match line.contains(&point){
            Ok(_) => return true,
            Err(_) => return false
        }
    }
}