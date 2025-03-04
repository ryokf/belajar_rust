fn main() {
    learn_10_memory_management();
}

fn learn_10_memory_management() {
    // rust membagi memory menjadi 2 bagian, stack dan heap
    // stack digunakan untuk menyimpan data yang ukurannya diketahui pada saat compile time
    // heap digunakan untuk menyimpan data yang ukurannya tidak diketahui pada saat compile time
    // stack lebih cepat dibandingkan heap karena stack menggunakan LIFO (Last In First Out) sedangkan heap menggunakan pointer
    // stack memiliki ukuran yang terbatas, sedangkan heap tidak terbatas
    
    // rust langsung menghapus data yang disimpan di stack ketika data tersebut tidak digunakan lagi
    // sedangkan data yang disimpan di heap tidak langsung dihapus ketika data tersebut tidak digunakan lagi
    // data yang disimpan di heap akan dihapus ketika pointer yang menunjuk ke data tersebut dihapus
    // pointer yang menunjuk ke data di heap disimpan di stack
}



