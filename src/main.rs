use crate::structs::Student;
mod enumtypes;
mod structs;
fn main() {

    let s = String::from("aaabbaaA");
    let result = equal(2,6);
    println!("The final result is {}", result);
    my_first_loop(6);
    loop_with_vector();
    extended_loop();
    //string_operations();
    counting_letters();
    println!("In this string there is a {} of a and A.", count_a(&s));

    let mut staud = Student {
        name: "Robert".to_string(),
        age: 21,
        heigh: 186,
        shoesize: 46,
    };
}

fn equal(a:i32, b:i32)->i32
{
    let mut c = a+b;
     c = c/2;
    c
}

fn my_first_loop(a:i32)
{
    for n in 0..a
        {
            println!("The number is {}", n)
        }

}

fn loop_with_vector()
{
    let mut v = Vec::new();
    v.push(7);
    v.push(21);
    v.push(41);
    v.push(0);

    let v1 = vec![12,5,3,51];

    for n in v
        {
            if n == 41
            {
                break;
            }
            println!("{}",n);
        }

    for n in v1
        {
            if n%2 == 0
            {
                continue;
            }
            println!("{}",n);
        }
}

fn extended_loop()
{
    let v = vec![1,14,2,4,1];
    'outer: for n in 0..10
        {
            for a in &v
                {
                    if n+a==17
                    {
                        break 'outer;
                    }
                    println!("Hello!, {}", a);
                }
        }
}

fn string_operations()
{
    let mut s = String::from("Barbara");
    let s1 = "Another hello!";
    s.push_str("UBARASZUNG"); //don't work in s1 case

    println!("S length is {}", s.len()); //number of bytes

    for n in s.chars()
        {
            println!("{}",n)
        }
    /*for n in  s.bytes()
        {
            println!("{}", n);
        }*/
    for n in s1.chars()
        {
            println!("{}",n)
        }
}

fn counting_letters()
{
    let s = String::from("Checking this.");

    for (i,c) in s.chars().enumerate()
        {
            println!("{} = {}", i,c);
        }
        //or
    for (i,c) in s.char_indices()
        {
            println!("{} = {}", i,c);
        }
}

fn count_a(s:&str)->i32
{
    let mut counter = 0;
    for c in s.chars()
        {
        if c == 'a' || c=='A'
        {
            counter +=1;
        }
    }
    counter
}