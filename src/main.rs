fn main() {
    learn_20_method();
}

fn learn_20_method() {
    struct Person{
        name: String,
        age: u8,
    }

    impl Person {
        fn say_hello(&self) {
            println!("Hello, my name is {}, and I am {} years old", self.name, self.age);
        }

        // static / associated method
        fn running(){
            println!("person is running");
        }
    }

    let p = Person {name: "John".to_string(), age: 20};
    p.say_hello();
    Person::running();
}
