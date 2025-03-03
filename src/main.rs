fn main() {
    learn_07_data_type_tuple();
}

fn learn_07_data_type_tuple() {
    // tipe data tuple
    let data: (i32, f64, bool) = (500, 6.4, true);
    println!("data: {:?}", data);

    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);

    // destructuring
    let (x, y, z) = data;
    println!("\nhasil destructuring");
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    // unit
    // unit adalah tuple yang tidak memiliki elemen
    // biasanya digunakan untuk mengembalikan nilai dari fungsi yang tidak mengembalikan nilai
    let unit: () = ();
    fn unit_func() -> () {
        println!("unit function");
    }
    println!("\nnilai dari fungsi unit_func(): {:?}", unit_func());
    println!("\nnilai dari variabel unit: {:?}", unit);
}



