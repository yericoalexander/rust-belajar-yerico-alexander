// Vector basics
pub fn basic_vector() {
    println!("\n--- Collection & Error Handling ---\n");

    println!("1. VECTOR (VEC)");
    println!("--------------");

    // Membuat vector baru
    println!("Cara membuat vector:");

    // Vector kosong dengan tipe
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);

    // Vector dengan macro vec!
    let v2 = vec![4, 5, 6];
    println!("v2: {:?}", v2);

    // Mengakses elemen vector
    println!("\nMengakses elemen vector:");

    // Dengan indeks (dapat panic jika indeks tidak valid)
    let third = &v1[2];
    println!("Elemen ketiga v1: {}", third);

    // Dengan get() (mengembalikan Option)
    match v1.get(2) {
        Some(value) => println!("Elemen ketiga v1 dengan get(): {}", value),
        None => println!("Tidak ada elemen pada indeks tersebut"),
    }

    // Iterasi pada vector
    println!("\nIterasi pada vector:");
    for i in &v1 {
        println!("Nilai: {}", i);
    }

    // Iterasi dengan perubahan
    println!("\nMengubah nilai dalam vector:");
    let mut v3 = vec![10, 20, 30];
    for i in &mut v3 {
        *i += 50; // Menggunakan dereference untuk mengubah nilai
    }
    println!("v3 setelah diubah: {:?}", v3);

    // Vector dengan berbagai tipe menggunakan enum
    println!("\nVector dengan berbagai tipe menggunakan enum:");

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Float(3.14),
        SpreadsheetCell::Text(String::from("Belajar Rust")),
    ];

    // Method-method vector yang berguna
    println!("\nMethod vector yang berguna:");
    let mut numbers = vec![1, 2, 3, 4, 5];

    // pop - menghapus dan mengembalikan elemen terakhir
    let last = numbers.pop();
    println!("Elemen terakhir: {:?}, numbers: {:?}", last, numbers);

    // len - panjang vector
    println!("Panjang vector: {}", numbers.len());

    // capacity - kapasitas alokasi vector
    println!("Kapasitas vector: {}", numbers.capacity());

    // clear - menghapus semua elemen
    numbers.clear();
    println!("Setelah clear(): {:?}, len: {}", numbers, numbers.len());
}

// HashMap basics
pub fn basic_hashmap() {
    use std::collections::HashMap;

    println!("\n2. HASHMAP");
    println!("---------");

    // Membuat HashMap baru
    println!("Cara membuat HashMap:");

    // HashMap kosong
    let mut scores = HashMap::new();

    // Menambahkan nilai
    scores.insert(String::from("Tim A"), 10);
    scores.insert(String::from("Tim B"), 50);

    println!("Scores: {:?}", scores);

    // Membuat HashMap dari vector
    println!("\nMembuat HashMap dari vector:");

    let teams = vec![String::from("Tim C"), String::from("Tim D")];
    let initial_scores = vec![20, 30];

    // Menggunakan zip dan collect
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("Scores2: {:?}", scores2);

    // Mengakses nilai
    println!("\nMengakses nilai dalam HashMap:");

    let tim_a = String::from("Tim A");

    // Dengan get (mengembalikan Option)
    match scores.get(&tim_a) {
        Some(score) => println!("Skor Tim A: {}", score),
        None => println!("Tim A tidak ditemukan"),
    }

    // Iterasi pada HashMap
    println!("\nIterasi pada HashMap:");

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting nilai
    println!("\nMenimpa nilai:");

    scores.insert(String::from("Tim A"), 25);
    println!(
        "Skor Tim A setelah diperbarui: {:?}",
        scores.get(&String::from("Tim A"))
    );

    // Insert jika key tidak ada (entry API)
    println!("\nMenyisipkan nilai hanya jika key belum ada:");

    scores.entry(String::from("Tim C")).or_insert(15);
    scores.entry(String::from("Tim A")).or_insert(15); // Tidak akan mengubah nilai Tim A

    println!("Scores setelah entry API: {:?}", scores);

    // Memperbarui nilai berdasarkan nilai lama
    println!("\nMemperbarui nilai berdasarkan nilai lama:");

    let text = "rust is fast rust is fun rust programming";
    let mut word_count = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word count: {:?}", word_count);
}

// Error handling dengan Result dan operator ?
pub fn result_and_question_mark() {
    use std::fs::File;
    use std::io::{self, ErrorKind, Read};

    println!("\n3. ERROR HANDLING DENGAN RESULT DAN ?");
    println!("----------------------------------");

    // Menggunakan Result<T, E>
    println!("Contoh penggunaan Result:");

    fn buka_file() -> Result<File, io::Error> {
        let file = File::open("hello.txt");

        match file {
            Ok(file) => Ok(file),
            Err(error) => Err(error),
        }
    }

    // Penggunaan match untuk menangani error
    println!("\nMenangani error dengan match:");

    fn buka_file_dengan_handling() -> Result<File, io::Error> {
        let file = File::open("hello.txt");

        match file {
            Ok(file) => Ok(file),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    println!("File tidak ditemukan, mencoba membuat file baru");
                    // Di sini biasanya kita akan membuat file dengan File::create
                    // tapi untuk contoh kita hanya mengembalikan error asli
                    Err(error)
                }
                other_error => {
                    println!("Error lain: {:?}", other_error);
                    Err(error)
                }
            },
        }
    }

    // Menggunakan unwrap dan expect
    println!("\nMenggunakan unwrap dan expect:");
    println!("- unwrap(): Mengembalikan nilai jika Ok, panic! jika Err");
    println!("- expect(): Seperti unwrap tetapi dengan pesan kustom");

    // let file = File::open("hello.txt").unwrap(); // akan panic! jika file tidak ada
    // let file = File::open("hello.txt").expect("Gagal membuka file hello.txt"); // panic! dengan pesan

    // Propagating errors dengan ?
    println!("\nOperator ? untuk propagating errors:");

    fn baca_username_dari_file() -> Result<String, io::Error> {
        let mut file = File::open("hello.txt")?; // ? akan return Err jika gagal
        let mut username = String::new();
        file.read_to_string(&mut username)?; // ? akan return Err jika gagal
        Ok(username)
    }

    // Chaining dengan ?
    println!("\nChaining dengan operator ?:");

    fn baca_username_singkat() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // ? dengan Option
    println!("\nOperator ? dengan Option:");

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    let result = last_char_of_first_line("Hello\nWorld");
    println!("Karakter terakhir baris pertama: {:?}", result);

    let empty_result = last_char_of_first_line("");
    println!("Karakter terakhir dari string kosong: {:?}", empty_result);
}

// Panic vs recoverable errors
pub fn panic_vs_recoverable() {
    println!("\n4. PANIC VS RECOVERABLE ERRORS");
    println!("-----------------------------");

    // Menggunakan panic!
    println!("panic!: Untuk kegagalan yang tidak dapat dipulihkan");
    println!("Contoh situasi menggunakan panic!:");
    println!("- Bug yang tidak mungkin terjadi dalam kode");
    println!("- Validasi input oleh pengguna");
    println!("- Kegagalan yang membuat program dalam kondisi tidak valid");

    // Contoh kode yang akan panic
    println!("\nKode yang akan panic (tidak dijalankan dalam contoh ini):");
    println!("- panic!(\"crash and burn\");");
    println!("- let v = vec![1, 2, 3]; v[99];");

    // Recoverable errors dengan Result
    println!("\nRecoverable errors dengan Result:");
    println!("- Digunakan untuk error yang wajar terjadi");
    println!("- Memungkinkan program untuk melanjutkan eksekusi");
    println!("- Contoh: file tidak ditemukan, koneksi jaringan terputus");

    // Contoh penggunaan Result untuk recoverable error
    println!("\nPenanganan recoverable error:");

    fn error_handling_example() {
        use std::fs::File;
        use std::io::{self, ErrorKind, Read};
        // Membuka file - operasi yang mungkin gagal
        let file_result = File::open("config.txt");

        let file = match file_result {
            Ok(file) => {
                println!("File ditemukan, melanjutkan operasi...");
                file
            }
            Err(error) => {
                println!("Tidak dapat membuka file: {:?}", error);
                println!("Menggunakan konfigurasi default...");
                return; // Keluar dari fungsi, tapi program tetap berjalan
            }
        };

        // Proses file di sini jika berhasil dibuka
    }

    // Kapan menggunakan masing-masing
    println!("\nPanduan memilih antara panic! dan Result:");
    println!("Gunakan panic! jika:");
    println!("- Situasi buruk dan tidak dapat dipulihkan");
    println!("- Ketika kode berada dalam kondisi yang rusak");
    println!("- Untuk kode contoh, prototipe, dan pengujian");

    println!("\nGunakan Result jika:");
    println!("- Kegagalan dapat diantisipasi (file tidak ada, dll)");
    println!("- Pengguna mungkin ingin menangani error");
    println!("- Operasi yang mungkin gagal pada kondisi normal (parser, validasi)");
}
