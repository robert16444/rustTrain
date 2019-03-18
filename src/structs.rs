use std::string::ToString;

pub struct Student
{
    name: String,
    age: i32,
    heigh: i32,
    shoesize: i32,
}

impl Student
{
    fn my_string(&self) -> String          //if you want change "self" -> &mut self
    {
        format!("{} - {} - {}cm - shoe:{}", self.name, self.age, self.heigh, self.shoesize)
    }

    fn grow(&mut self, h: i32)
    {
        self.heigh += h;
    }

    fn die(self)
    {
        println!("Dead{}", self.my_string());
    }
}

fn main() {
    let mut stud = Student {
        name: "Robert".to_string(),
        age: 21,
        heigh: 186,
        shoesize: 46,
    };
    println!("Student data is: {}", stud.my_string());
    stud.grow(10);
    println!("Student data is: {}", stud.my_string());
    stud.die();
}
