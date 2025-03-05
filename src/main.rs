fn main() {
    learn_15_function();
}

fn learn_15_function() {
    // Function
    // penulisan nama function di rust menggunakan snake_case
    fn nama_function() {
        println!("Hello World");
    }
    nama_function();

    // Function dengan parameter
    // parameter harus menuliskan tipe data
    fn say_hello(nama: &str) {
        println!("Hello {}", nama);
    }
    say_hello("Rust");

    // Function dengan return value
    fn tambah(a: i32, b: i32) -> i32 {
        a + b // tidak perlu menuliskan kata return
    }
    let hasil = tambah(10, 20);
    println!("Hasil tambah: {}", hasil);
}
