use std::{fmt::Display, ops::{Add, Sub}};


trait GetArea{
    fn get_area(&self) -> Area;
}
struct Point{
    x:i32,
    y:i32,
}
impl Default   for Point{
    fn default() -> Self {
        Point { x: 0, y: 0}
    }
}
impl Add for Point{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x:self.x + rhs.x,
            y:self.y + rhs.y,
        }
    }
}
impl Sub for Point{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x:self.x - rhs.x,
            y:self.y - rhs.y,
        }
    }
}
impl GetArea   for Point{
    fn get_area(&self) -> Area {
        Area { area: Area::default().area}
    }
}

struct Circle{
    radius: i32,
    center:Point
}
impl Default for Circle{
    fn default() -> Self {
        Circle { radius: 1, center: Point::default()}
    }
}
impl GetArea for Circle{
    fn get_area(&self) -> Area {
        let radius = self.radius as f32;
        Area{area: 3.14*radius * radius}
    }
}

struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}
impl Default for Rectangle{
    fn default() -> Self {
        Rectangle { top_left: Point { x: -1, y: 1 }, bottom_right: Point { x: 1, y: -1 } }
    }
}
impl GetArea for Rectangle{
    fn get_area(&self) -> Area {
        let width = self.top_left.x-self.bottom_right.x;
        let height = self.top_left.y-self.bottom_right.y;
        return Area{area:(width*height) as f32 };
    }
}

struct Area{
    area:f32
}
impl Default for Area{
    fn default() -> Self {
        Area { area: 0.0 }
    }
}
impl Display for Area{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {} cm²", self.area)
    }
}
impl Add for Area{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            area: self.area + rhs.area
        }
    }
}

// impl Add for &dyn GetArea{
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self::Output {
//         Self{
//             area: self.get_area() + rhs.get_area()
//         }
//     }
// }

// fn sum_area(areas: &[&dyn GetArea]) -> Area{
//     let area = Area::default();
//     for obj in areas.iter(){
//         area = area + obj;
//     }
//     return area;
// }

pub fn main_es6(){


}