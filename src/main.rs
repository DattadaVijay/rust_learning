struct Person {
    name: String,
    age: i32,
}

enum TrafficLight{
    Red,
    Yellow,
    Green,
}

enum Direction{
    North,
    East,
    West,
    South,
}

impl Person{

    fn new_person(name: String, age:i32) -> Person {
        Person{
             name: name,
             age: age,
        }

    }

    fn print_person(&self) {
    println!("Name of the person is {} and he is {} years old", self.name, self.age);
}

    fn increment_age(&mut self){
        self.age += 1;
}

}

fn main() {

    let mut person = Person::new_person("Vijay".to_string(), 31);
    let light_status = TrafficLight::Red;
    let direction = Direction::East;

    match direction{
        Direction::North => println!("Going East"),
        _ => println!("Going somewhere else"),
    }

    match light_status{
        TrafficLight::Green => println!("Go"),
        TrafficLight::Yellow => println!("Prepare to Stop"),
        TrafficLight::Red => println!("Stop"),
    }

    person.print_person();
    person.increment_age();
    person.print_person();

    if person.age>30{
        println!("The incremental function worked")
    }else{
        println!("The incremental function did not work")
    }
    
}