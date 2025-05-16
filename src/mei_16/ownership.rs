pub fn ownership_rules() {
    println!("\n--- Ownership, Borrowing & Lifetimes ---\n");

    println!("--- OWNERSHIP RULES ---");
    println!("Rust memiliki 3 aturan ownership:");
    println!("1. Setiap nilai memiliki variabel yang disebut 'owner'");
    println!("2. Hanya boleh ada satu owner pada satu waktu");
    println!("3. Ketika owner keluar dari scope, nilai akan di-drop (dihapus dari memori)");

    // Contoh 1: Ownership dan Move
    println!("\nContoh Move:");
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // Error! s1 sudah dipindahkan ke s2
    println!("s2: {}", s2); // Ini valid

    // Contoh 2: Clone (deep copy)
    println!("\nContoh Clone:");
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4); // Keduanya valid karena s4 adalah clone dari s3

    // Contoh 3: Copy untuk tipe primitif (stack-only)
    println!("\nContoh Copy:");
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y); // Keduanya valid karena tipe integer diimplementasikan trait Copy
}

// 2. Borrowing (&, &mut)
pub fn borrowing() {
    println!("\n--- BORROWING ---");
    println!("Borrowing adalah meminjam referensi tanpa mengambil kepemilikan");
    println!("Ada dua jenis borrowing:");
    println!("1. Immutable borrows (&T) - Bisa memiliki banyak referensi");
    println!("2. Mutable borrows (&mut T) - Hanya boleh satu referensi pada satu waktu");

    // Contoh 1: Immutable borrowing
    println!("\nContoh Immutable Borrowing:");
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("Panjang dari '{}' adalah {}.", s, len);

    // Contoh 2: Mutable borrowing
    println!("\nContoh Mutable Borrowing:");
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("String setelah diubah: {}", s);

    // Contoh 3: Aturan borrowing
    println!("\nAturan Borrowing:");
    let mut s = String::from("hello");

    // Ini akan berjalan baik
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);
    // r1 dan r2 tidak digunakan lagi setelah baris ini

    // Sekarang kita bisa membuat mutable reference
    let r3 = &mut s;
    println!("r3: {}", r3);
}

// Fungsi untuk contoh immutable borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Fungsi untuk contoh mutable borrowing
fn append_world(s: &mut String) {
    s.push_str(" world");
}

// 3. String vs &str
pub fn string_vs_str() {
    println!("\n--- STRING VS &str ---");
    println!("String: Tipe yang dimiliki, dapat diubah, dialokasikan di heap");
    println!("&str: Slice dari String, tidak dapat diubah, referensi ke data String");

    // Contoh String
    println!("\nContoh String:");
    let mut s = String::from("hello world");
    s.push_str(", Rust!"); // String bisa dimodifikasi
    println!("String: {}", s);

    // Contoh &str (string slice)
    println!("\nContoh &str (string slice):");
    let s = "hello world"; // &str literal
    println!("&str literal: {}", s);

    let string = String::from("hello world");
    let slice = &string[0..5]; // &str slice dari String
    println!("slice dari String: {}", slice);

    // Konversi
    println!("\nKonversi antara String dan &str:");
    let s1: &str = "hello";
    let s2: String = s1.to_string(); // &str -> String
    let s3: String = String::from(s1); // &str -> String
    let s4: &str = &s3; // String -> &str (borrowing)
    println!("s1: {}, s2: {}, s3: {}, s4: {}", s1, s2, s3, s4);
}

// 4. Lifetime konsep dasar
pub fn lifetime_basics() {
    println!("\n--- LIFETIME BASICS ---");
    println!("Lifetime adalah cara Rust menjamin bahwa referensi tetap valid");
    println!("Anotasi lifetime diawali dengan tanda apostrof (')");

    // Contoh fungsi dengan parameter lifetime
    println!("\nContoh fungsi dengan lifetime explicit:");
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, string2);
    println!("String terpanjang: {}", result);

    // Contoh struct dengan lifetime
    println!("\nContoh struct dengan lifetime:");
    let novel = String::from("Panggil aja aku Rico. Beberapa tahun yang lalu...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {}", i.part);
}

// Fungsi dengan parameter lifetime explicit
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct dengan lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}
