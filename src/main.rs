fn main() {
    learn_16_function_ownership();
}

fn learn_16_function_ownership() {
    fn test_func_data_stack(score: i32) {
        println!("score di dalam function dari stack: {}", score);
    }

    fn test_func_data_heap(data: String) {
        println!("data di dalam function dari heap: {}", data);
    }

    // data yang berada di stack ketika di passing ke function
    // akan di copy ke dalam function tersebut
    let angka = 10;
    test_func_data_stack(angka);
    println!("data angka masih bisa diakses : {}", angka);

    // data yang berada di heap ketika di passing ke function
    // akan di move ke dalam function tersebut
    let data = String::from("Hello World");
    test_func_data_heap(data);
    // println!("data tidak bisa diakses : {}", data); // error

    // ownership return value
    fn test_func_return_value(value1: String, value2: String) -> String {
        format!("{} {}", value1, value2)
    }

    let value1 = String::from("Hello");
    let value2 = String::from("World");
    let result = test_func_return_value(value1, value2); //ownership dari variable value1 dan value2 di move ke result
    println!("hasil return function test_func_return_value: {}", result);
    // println!("value1: {}", value1); // error
    // println!("value2: {}", value2); // error

    // mengembalikan ownership
    fn test_func_return_ownership(x: String, y: String) -> (String, String, String) {
        let result = format!("{} {}", x, y);
        return (result, x, y);
    }

    let x = String::from("Hello");
    let y = String::from("World");
    let (result, x, y) = test_func_return_ownership(x, y);
    println!("result: {}", result);
    println!("x: {}", x);
    println!("y: {}", y);

}
