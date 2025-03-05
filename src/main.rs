fn main() {
    learn_17_references_and_borrowing();
}

fn learn_17_references_and_borrowing() {
    // dengan references kita bisa mengirimkan data ke function tanpa perlu mengirimkan data aslinya
    // sehingga kita masih bisa menggunakan data aslinya
    // proses ini disebut borrowing
    // references menggunakan simbol &
    fn full_name(first_name: &String, last_name: &String) -> String {
        format!("{} {}", first_name, last_name)
    }
    let x = String::from("John");
    let y = String::from("Doe");
    let z = full_name(&x, &y);
    println!("fisrt name: {}", x);
    println!("last name: {}", y);
    println!("full name: {}", z);

    // secara default, references bersifat immutable
    // sehingga kita tidak bisa mengubah data yang di reference
    #[allow(unused)]
    fn ubah_nama_error(nama: &String) {
        // nama.push_str(" Doe"); // error
    }

    // jika kita ingin mengubah data yang di reference, kita bisa menggunakan mutable references
    // mutable references menggunakan simbol &mut
    fn ubah_nama(nama: &mut String) {
        nama.push_str(" Doe");
    }
    let mut nama = String::from("John"); // data asli yang direference harus mutable
    ubah_nama(&mut nama);
    println!("data yang diubah: {}", nama);

    // mutable references hanya bisa ada satu pada satu waktu
    // jika kita membuat dua mutable references, maka akan terjadi error
    let mut nama = String::from("John");
    #[allow(unused)]
    let mut nama1 = &mut nama;
    // let mut nama2 = &mut nama; // error: cannot borrow `nama` as mutable more than once at a time
    println!("nama1: {}", nama1);
}
