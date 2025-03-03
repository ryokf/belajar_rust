fn main() {
    learn_04_data_type_number();
}

fn learn_04_data_type_number() {
    // tipe data default integer adalah i32
    // tipe data default float adalah f64

    // Integer
    let a: i8 = 127;
    let b: i16 = 32767;
    let c: i32 = 2147483647;
    let d: i64 = 9223372036854775807;
    let e: i128 = 170141183460469231731687303715884105727;
    let f: isize = 9223372036854775807;
    let g: u8 = 255;
    let h: u16 = 65535;
    let i: u32 = 4294967295;
    let j: u64 = 18446744073709551615;
    let k: u128 = 340282366920938463463374607431768211455;
    let l: usize = 18446744073709551615;

    // Float
    let m: f32 = 3.40282346638528859811704183484516925440;
    let n: f64 = 1.797693134862315708145274237317043567981e308;

    println!("nilai maksimum i8: {}", a);
    println!("nilai maksimum i16: {}", b);
    println!("nilai maksimum i32: {}", c);
    println!("nilai maksimum i64: {}", d);
    println!("nilai maksimum i128: {}", e);
    println!("nilai maksimum isize: {}", f);
    println!("nilai maksimum u8: {}", g);
    println!("nilai maksimum u16: {}", h);
    println!("nilai maksimum u32: {}", i);
    println!("nilai maksimum u64: {}", j);
    println!("nilai maksimum u128: {}", k);
    println!("nilai maksimum usize: {}", l);
    println!("nilai maksimum f32: {}", m);
    println!("nilai maksimum f64: {} \n", n);

    // konversi tipe data
    println!("contoh konversi tipe data yang valid");
    let o: i32 = 10;
    println!("nilai o dengan tipe data i32: {}", o);
    let p: f64 = o as f64;
    println!("nilai o dengan tipe data f64: {}", p);
    
    println!("");

    // konversi tipe data yang tidak valid menyebabkan integer overflow
    println!("contoh konversi tipe data yang tidak valid");
    let q: i32 = 1000;
    println!("nilai q dengan tipe data i32: {}", q);
    let r: i8 = q as i8;
    println!("nilai q dengan tipe data i8: {}", r); //nilai r menjadi -24
}



