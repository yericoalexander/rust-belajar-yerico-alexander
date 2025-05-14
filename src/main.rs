fn main() {
    println!("Belajar Rust - 14 Mei 2025");

    let x = 42;  // immutable nilainya tidak bisa diubah setelah didefinisikan
    let mut y = 7;  // mutable Mutable Variable (Bisa Diubah)
    y = y * 2;  // mengubah nilai y
    println!("Nilai x: {}, Nilai y setelah diubah: {}", x, y);

    // tipe data primitive
    let tup: (i32, f64, &str) = (100, 3.14, "Rust");
    let arr = [10, 20, 30, 40, 50];
    
    // Mengakses elemen tuple dan array
    println!("Elemen tuple: ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("Elemen array: [{}]", arr.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
    
    // Menghitung jumlah elemen array
    let sum: i32 = arr.iter().sum();
    println!("Jumlah semua elemen arrayyyyy: {}", sum);
}