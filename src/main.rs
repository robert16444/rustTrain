extern crate rand;
use rand::Rng;
use rand::random;
use std::ops::Add;

mod enumtypes;
mod structs;

#[derive(Debug, Clone, Copy)]
pub struct Point
{
    x:i32,
    y:i32
}

impl Point
{
    fn random()->Self
    {
        let mut tr = rand::thread_rng();
        Point
            {
                x:tr.gen(),
                y:tr.gen(),
            }
    }

}

impl Add for Point
{
    type Output = Point;
    fn add(self,other:Point)->Self::Output
    {
        Point
            {
                x:self.x + other.x,
                y:self.y + other.y,
            }
    }
}

fn main() {
    let x:u8 = random();
    let d = Point::random();
    println!("{:?}",d);

}
