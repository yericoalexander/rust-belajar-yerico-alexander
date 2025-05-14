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
    println!("Welcome to Rust programming!");

    let x = 42;  // immutable
    let mut y = 7;  // mutable
    y = y * 2;  // mengubah nilai y
    println!("Nilai x: {}, Nilai y setelah diubah: {}", x, y);

    // tipe data primitive
    let tup: (i32, f64, &str) = (100, 3.14, "Rust");
    let arr = [10, 20, 30, 40, 50];
    
    println!("Elemen tuple: ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("Elemen array: [{}]", arr.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
    
    let sum: i32 = arr.iter().sum();
    println!("Jumlah semua elemen array: {}", sum);
}
```

## 🎯 Output Program:
```
Welcome to Rust programming!
Nilai x: 42, Nilai y setelah diubah: 14
Elemen tuple: (100, 3.14, Rust)
Elemen array: [10, 20, 30, 40, 50]
Jumlah semua elemen array: 150
```

## ✅ 15 Mei 2025 Saya Belajar:
