# 📝 Progress Belajar Rust 

## ✅ 14 Mei 2025 Saya Belajar:

- **Variabel di Rust**
  - Immutable (default): `let x = 42;`
  - Mutable: `let mut y = 7;`

- **Tipe Data Dasar**
  - Tuple: `let tup: (i32, f64, &str) = (100, 3.14, "Rust");`
  - Array: `let arr = [10, 20, 30, 40, 50];`

- **Mengakses & Manipulasi Data**
  - Akses elemen tuple: `tup.0`, `tup.1`
  - Iterasi array: `arr.iter()`
  - Fungsi agregasi: `arr.iter().sum()`

## 💻 Kode Program:

```rust
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
    println!("Jumlah semua elemen array: {}", sum);
}
```

## 🎯 Output Program:
```
Belajar Rust - 14 Mei 2025
Nilai x: 42, Nilai y setelah diubah: 14
Elemen tuple: (100, 3.14, Rust)
Elemen array: [10, 20, 30, 40, 50]
Jumlah semua elemen array: 150
```

## ✅ 15 Mei 2025 Saya Belajar:

- **Percabangan**
  - If-else: = `if kondisi { } else { }`
  - Match: `match nilai { pola => hasil, ... }`

- **Looping**
  - Loop tanpa batas: `loop { }`
  - While: `while kondisi { }`
  - For: `for item in koleksi { }`

- **Function & Parameter**
  - Deklarasi: `fn nama_fungsi(param: type) -> return_type { }`
  - Return value: nilai terakhir tanpa semicolon `;`

- **Scope & Shadowing**
  - Scope: variabel hanya hidup dalam blok `{ }`
  - Shadowing: mendeklarasi ulang variabel dengan nama sama


## 💻 Kode Program:

```rust
fn main() {
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
```

## 🎯 Output Program:
```
--- Control Flow & Functions ---

1. Percabangan if dan match:
Nilai B
Match expression: 3 -> Tiga

2. Looping: loop, while, for:
Loop dengan break: 10
While loop: 3
While loop: 2
While loop: 1
For loop dengan range:
  Iterasi 1
  Iterasi 2
  Iterasi 3
For loop dengan array:
  Elemen: 10
  Elemen: 20
  Elemen: 30
  Elemen: 40
  Elemen: 50

3. Function dan Parameter:
Hasil tambah: 15
Halo Rustacean! Umur kamu 25 tahun.

4. Scope dan Shadowing:
Nilai dalam scope: 200
Variabel dalam: 300
Nilai luar scope: 100
Data (string): 42
Data (integer): 42
```

## ✅ 16 Mei 2025 Saya Belajar:
