fn main() {
    learn_08_data_type_array();
}

fn learn_08_data_type_array() {
    // tipe data array
    // array adalah kumpulan data yang memiliki tipe data yang sama
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    // akses data array
    let first = a[0];
    let second = a[1];
    println!("first: {}, second: {}", first, second);

    // panjang array
    // panjang array bertipe data usize karena maksimal panjang array sesuai dengan ukuran memori di sistem operasi
    let len = a.len();
    println!("len: {}", len);

    // array 2 dimensi
    let b: [[i32; 2]; 3] = [
        [1, 2],
        [3, 4],
        [5, 6]
    ];
    println!("b: {:?}", b);
}



