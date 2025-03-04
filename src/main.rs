fn main() {
    learn_11_data_type_string();
}

fn learn_11_data_type_string() {
    // rust membagi string menjadi dua jenis, yaitu &str dan String
    // &str adalah tipe data yang tidak bisa diubah, sedangkan String adalah tipe data yang bisa diubah
    // &str disimpan di stack, sedangkan String disimpan di heap

    let a = "ini adalah string menggunakan &str"; // &str
    let b = String::from("ini adalah string menggunakan String"); // String

    println!("{}", a);
    println!("{}", b);
}



