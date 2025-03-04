fn main() {
    learn_12_ownership();
}

fn learn_12_ownership() {
    // Ownership
    // rust memiliki konsep memory management yang unik, yaitu ownership
    // ownership adalah konsep yang memungkinkan rust untuk memastikan bahwa memory yang digunakan
    // tidak akan bocor atau tidak terjadi memory leak

    // contoh pada penyimpanan data pada stack
    let x = 5;
    let y = x; // copy value, y mengambil nilai dari x tidak terjadi ownership movement
    println!("x = {}, y = {}", x, y); // sehingga variabel x masih bisa digunakan

    // contoh pada penyimpanan data pada heap
    let s1 = String::from("hello");
    let s2 = s1; // ownership movement, s2 mengambil alamat memory s1
    // println!("s1 = {}, s2 = {}", s1, s2); // error, karena s1 sudah tidak valid lagi, karena s2 sudah mengambil alamat memory s1
    println!("nilai s2 = {}", s2);

    // jika ingin mengcopy data pada heap, maka gunakan clone
    let s3 = s2.clone(); // tidak terjadi ownership movement
    println!("s2 = {}, s3 = {}", s2, s3);
}



