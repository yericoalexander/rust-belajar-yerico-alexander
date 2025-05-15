fn main() {
    println!("Belajar Rust - 14-19 Mei 2025");

    // 14 MEI : VARIABEL DAN TIPE DATA DASAR
    println!("\n--- VARIABEL DAN TIPE DATA DASAR ---\n");
    
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
    println!("Jumlah semua elemen array: {}", sum);

  
    // 15 MEI : CONTROL FLOW & FUNCTIONS
    println!("\n--- Control Flow & Functions ---");
    
    // 1. Percabangan if dan match
    println!("\n1. Percabangan if dan match:");
    
    // Contoh if-else
    let nilai = 75;
    if nilai >= 80 {
        println!("Nilai A");
    } else if nilai >= 70 {
        println!("Nilai B");
    } else {
        println!("Nilai C");
    }
    
    // Contoh match
    let angka = 3;
    let hasil = match angka {
        1 => "Satu",
        2 => "Dua",
        3 => "Tiga",
        _ => "Lainnya",
    };
    println!("Match expression: {} -> {}", angka, hasil);

    // 2. Looping: loop, while, for
    println!("\n2. Looping: loop, while, for:");
    
    // loop dengan break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("Loop dengan break: {}", result);
    
    // while loop
    let mut number = 3;
    while number != 0 {
        println!("While loop: {}", number);
        number -= 1;
    }
    
    // for loop dengan range
    println!("For loop dengan range:");
    for i in 1..4 {
        println!("  Iterasi {}", i);
    }
    
    // for loop dengan array
    println!("For loop dengan array:");
    for elem in arr.iter() {
        println!("  Elemen: {}", elem);
    }
    
    // 3. Function dan Parameter
    println!("\n3. Function dan Parameter:");
    
    // Memanggil function
    println!("Hasil tambah: {}", tambah(10, 5));
    
    // Function dengan multiple parameters
    let pesan = buat_pesan("Rustacean", 25);
    println!("{}", pesan);
    
    // 4. Scope dan Shadowing
    println!("\n4. Scope dan Shadowing:");
    
    // Scope dengan block
    let luar = 100;
    {
        let luar = 200;  // Shadowing
        println!("Nilai dalam scope: {}", luar);
        
        let dalam = 300;
        println!("Variabel dalam: {}", dalam);
    }
    println!("Nilai luar scope: {}", luar);
    // println!("Variabel dalam: {}", dalam);  // Error: dalam tidak tersedia di luar scope
    
    // Shadowing dengan tipe data berbeda
    let data = "42";
    println!("Data (string): {}", data);
    
    let data = data.parse::<i32>().unwrap();  // Shadowing dengan tipe berbeda
    println!("Data (integer): {}", data);
}

// Function dengan return value
fn tambah(a: i32, b: i32) -> i32 {
    // Return implicit (tanpa keyword return)
    a + b
}

// Function dengan multiple parameters
fn buat_pesan(nama: &str, umur: u32) -> String {
    return format!("Halo {}! Umur kamu {} tahun.", nama, umur);
}