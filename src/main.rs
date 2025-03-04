fn main() {
    learn_09_constant();
}

fn learn_09_constant() {
    // constant selalu immutable
    // constant harus di-annotasi dengan tipe data
    // penamaan constant menggunakan huruf kapital dan dipisa dengan underscore
    const MAX_POINTS: i32 = 100000;
    // const MIN_POINTS = 0; // error: cannot determine a type for this constant
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
    // MAX_POINTS = 100_001; // error: cannot assign to this expression
    
}



