use std::ops::Add;

#[derive(Debug)]
pub struct Point
{
    x:i32,
    y:i32
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
    let a = Point{x:3, y:4};
    let b = Point{x:24, y:14};

    let c = a+b;
    println!("c = {:?}",c);
}

////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////

#[derive(PartialEq,Debug)]
pub struct USD(i32);
#[derive(PartialEq,Debug)]
pub struct GBP(i32);
#[derive(PartialEq,Debug)]
pub struct PLN(i32);

pub trait ToUSD
{
    fn to_usd(&self)->USD;
    fn convert<T:FromUSD>(&self)->T
    {
        T::from_usd(&self.to_usd())
    }
}

impl ToUSD for GBP
{
    fn to_usd(&self)->USD
    {
        USD((self.0 * 130)/100)
    }
}

pub trait FromUSD
{
    fn from_usd(u:&USD)->Self;
}

impl FromUSD for PLN
{
    fn from_usd(u:&USD)->Self
    {
        PLN((u.0 * 130)/100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let g = GBP(200);
        let u = g.to_usd();
        assert_eq!(u, USD(260));
        let c = PLN::from_usd(&u);
        assert_eq!(c, PLN(338));
        let c2:PLN = g.convert();
        assert_eq!(c2,c);
    }
}
