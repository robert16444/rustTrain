use std::collections::HashMap;
use std::env::args;

fn main()
{
    let mut hm = HashMap::new();
    hm.insert(3, "hello");
    hm.insert(5, "world");

    let r = match hm.get(&3)
        {
            Some(v)=>v,
            _=>"NOTHING",
        };
    println!("{}", r);

    match "3".parse::<f32>()       //if we put f.e. 3t here we'll have got error -> we can't pras it to f32
        {
            Ok(v)=>println!("OK - {}", v),
            Err(e)=>println!("Errror - {}", e),
        }
//on this -> match "3".parse::<f32>() -> we can also use .unwarp and unwarp_or -> match "3".parse::<f32>().unwrap_or

    let s = hm.get(&3).unwrap();
    //let t = hm.get(&4).unwrap();
    let u = hm.get(&4).unwrap_or(&"nothing data here");
    println!("{}", s);
    //println!("{}", t); // Don't work. That the reason why the "match" way with Some and None is better
    println!("{}", u);

    fn get_arg(n:usize)->Result<String,String> {
        for (i, a) in args().enumerate() {
            if (i == n) {
                return Ok(a)
            }
        }
        Err("error args".to_string())

        match get_arg(3)
            {
                Ok(v) => println!("OK - {}", v),
                Err(e) => println!("Errror - {}", e),
            }
    }
    //this funcion return name with first letter w/W
    /*fn main() {
        for a in args()
            {
                if let Some(c) = a.chars().next()
                {
                    match c
                        {
                            'w'|'W' => println!("Hello {}!",a),  //cargo run -- wally mat igor Wiesiek
                            _=>{}
                        }
                }
            }
    }*/
}