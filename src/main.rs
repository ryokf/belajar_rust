fn main() {
    learn_23_trait();
}

fn learn_23_trait() {
    struct Person{
        name: String,
        age: u8
    }

    trait CanSayHello{
        fn hello(&self){
            println!("hello");
        }
        fn say_hello(&self);
        fn say_hello_to(&self, name: &str);
    }

    impl CanSayHello for Person{
        fn say_hello(&self){
            println!("Hello my name is {}", self.name);
        }

        fn say_hello_to(&self, name: &str){
            println!("Hello {}, my name is {}", name, self.name);
        }
    }

    let p = Person{name: "test".to_string(), age: 22};
    p.hello();
    p.say_hello();
    p.say_hello_to("bob");
}
