fn main() {
    // learn_01_hello_world();
    // learn_02_variable();
    learn_03_data_type();
}

#[allow(dead_code)]
fn learn_01_hello_world() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn learn_02_variable() {
    // menggunakan let untuk mendeklarasikan variabel bersifat immutable (tidak bisa diubah)
    let x = 5;
    let y = 10;
    let z = x + y;
    // x = 10; // error: cannot assign twice to immutable variable
    println!("The value of x is: {} and y is: {}", x, y);
    println!("The value of z is: {} + {} = {}", x, y, z);

    // menggunakan let mut untuk mendeklarasikan variabel bersifat mutable (bisa diubah)
    let mut a = 5;
    println!("The initial value of a is: {}", a);
    a = 10;
    println!("The new value of a is: {}", a);

    //static typing
    let mut name = "Ryo";
    println!("My name is {}", name);
    name = "Ryo Kf";
    // name = 20; // error: expected `&str`, found integer
    println!("My name now is {}", name);

    // shadowing (menggunakan nama variabel yang sama)
    let language = "Rust";
    print!("I learned {} ", language);
    let language = 1;
    println!("in {} year", language);


}

#[allow(dead_code)]
fn learn_03_data_type() {
    // tipe data bisa diatur secara eksplisit
    let a: i32 = 5; // i32 adalah tipe data integer 32-bit
    println!("The value of a is: {}", a);

    // tipe data bisa diatur secara implisit
    let b = 10; // Rust akan menentukan tipe data dari nilai yang diberikan
    println!("The value of b is: {}", b);
}



