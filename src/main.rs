fn main() {
    learn_19_struct();
}

fn learn_19_struct() {
    struct Person {
        name: String,
        age: i8,
        is_male: bool,
    }

    let person1 = Person {
        name: String::from("John"),
        age: 25,
        is_male: true,
    };

    println!("Name: {}", person1.name);
    println!("Age: {}", person1.age);
    println!("Is male: {} \n", person1.is_male);

    // struct dengan function
    fn print_person(person: &Person) {
        println!("data dari print_person");
        println!("Name: {}", person.name);
        println!("Age: {}", person.age);
        println!("Is male: {} \n", person.is_male);
    }
    print_person(&person1);

    //shorthand
    let name = String::from("Doe");
    let age = 30;
    let is_male = true;
    let person2 = Person { name, age, is_male };
    print_person(&person2);

    // struct update syntax
    let person3 = Person{..person1};
    print_person(&person3);
    // print!("Name: {}", person1.name); // error: value borrowed here after move

    // struct tuple
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("Color: {}, {}, {}", black.0, black.1, black.2);
}
