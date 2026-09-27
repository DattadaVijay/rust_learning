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

    let mut count = 0;

    loop{

        println!("The loop counter is {}", count);
        count +=1;
        if count == 5{
           break;
        }

    }

    let mut count = 0;

    while count<5{
        println!("The while loop counter is {}", count);
        count+=1;
    }

    for i in 1..5{
        println!("for loop counter is {}",i)
    }
    
    // arrays generally dont grow or shrink it should be vec! for that
    let mut numbers:[i32;4] = [10, 20, 30, 40];
    println!("numbers array  =  {:?}", numbers);

    let mut vec_numbers = vec![10, 20, 30, 40];
    vec_numbers.push(50);
    //we also can insert at specific loc in array
    vec_numbers.insert(2, 21);
    println!("vec_numbers = {:?}", vec_numbers);

}