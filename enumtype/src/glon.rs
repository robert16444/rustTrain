#[derive(Debug)]
pub struct Bed
{
    size:i32,
    count:u32,
}

#[derive(Debug)]
pub enum Room
{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge,
}

fn main() {
    //let t = Room::Kitchen(4);
    //or
    //let b = Room::Bedroom(Bed{size:200,count:3});

    //or

      use self::Room::*;
      let t = Kitchen(4);

    println!("Hello from there {:?}!",t);   // :? is here because of Debug

   /* match t
        {
            Kitchen(n) => println!("The kitchen have {:?} windows", n),
            a=>println!("{:?}",a),
        }*/

   /* let v =  match t
        {
            Kitchen(n) => n,
            d=> 0,
        } + 10;
    println!("v={}",v);*/

    if let Kitchen(n) = t
    {
        println!("The kitchen have {} windows",n);
    }
}
