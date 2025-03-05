fn main() {
    learn_14_for_loop();
}

fn learn_14_for_loop() {
    let arr = [10, 20, 30, 40, 50];
    
    // for loop bisa digunakan untuk mengiterasi array
    for i in arr {
        println!("iterasi array : {}", i);
    }

    println!();

    // rust memiliki tipe data Range
    // Range adalah tipe data yang berisi rentang angka
    // Range bisa digunakan untuk mengiterasi angka
    let range = 1..6;
    for i in range {
        println!("iterasi menggunakan range: {}", i);
    }
}
