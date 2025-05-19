 // Struct dasar
        pub fn basic_struct() {
            println!("\n--- Struct, Enum, & Pattern Matching ---\n");

            println!("1. STRUCT DASAR");
            println!("--------------");
            
            // Definisi struct
            struct User {
                username: String,
                email: String,
                active: bool,
            }
            
            // Membuat instance struct
            let user1 = User {
                username: String::from("Rico"),
                email: String::from("yericoalexander12@gmail.com"),
                active: true,
            };
            
            // Mengakses data
            println!("Username: {}", user1.username);
            println!("Email: {}", user1.email);
            println!("Status: {}", if user1.active { "aktif" } else { "tidak aktif" });
            
            // Struct tuple
            struct Point(i32, i32);
            let point = Point(10, 20);
            println!("Koordinat: ({}, {})", point.0, point.1);
        }
        
        // Implementasi method pada struct
        pub fn impl_struct() {
            println!("\n2. STRUCT DENGAN IMPLEMENTASI");
            println!("----------------------------");
            
            struct Rectangle {
                width: u32,
                height: u32,
            }
            
            // Implementasi method
            impl Rectangle {
                // Constructor
                fn new(width: u32, height: u32) -> Rectangle {
                    Rectangle { width, height }
                }
                
                // Method untuk menghitung luas
                fn area(&self) -> u32 {
                    self.width * self.height
                }
            }
            
            // Menggunakan struct dengan method
            let rect = Rectangle::new(30, 50);
            println!("Luas persegi panjang: {}", rect.area());
        }

        // Enum dasar
        pub fn basic_enum() {
            println!("\n3. ENUM DASAR");
            println!("------------");
            
            // Definisi enum
            enum Message {
                Quit,                       // Tanpa data
                Move { x: i32, y: i32 },    // Struct-like
                Write(String),              // Tuple-like
                Color(i32, i32, i32),       // Tuple dengan banyak nilai
            }
            
            // Membuat instance enum
            let msg1 = Message::Quit;
            let msg2 = Message::Move { x: 10, y: 20 };
            let msg3 = Message::Write(String::from("Hello"));
            let msg4 = Message::Color(255, 0, 0);
            
            // Enum dengan method
            impl Message {
                fn call(&self) {
                    println!("Memanggil pesan");
                }
            }
            
            msg3.call();
        }
        
        // Match expression dengan enum
        pub fn match_example() {
            println!("\n4. ENUM DENGAN MATCH");
            println!("-------------------");
            
            enum Coin {
                Penny,
                Nickel,
                Dime,
                Quarter,
            }
            
            // Match untuk enum
            fn value_in_cents(coin: Coin) -> u8 {
                match coin {
                    Coin::Penny => 1,
                    Coin::Nickel => 5,
                    Coin::Dime => 10,
                    Coin::Quarter => 25,
                }
            }
            
            println!("Penny bernilai {} cents", value_in_cents(Coin::Penny));
            println!("Quarter bernilai {} cents", value_in_cents(Coin::Quarter));
            
            // Match dengan nilai
            let angka = 3;
            match angka {
                1 => println!("Satu"),
                2 => println!("Dua"),
                3 => println!("Tiga"),
                _ => println!("Lainnya"),
            }
        }

        // Option enum
        pub fn option_example() {
            println!("\n5. OPTION ENUM");
            println!("-------------");
            
            // Option<T> adalah enum bawaan Rust:
            // enum Option<T> {
            //     Some(T),
            //     None,
            // }
            
            // Menggunakan Option
            let ada = Some(5);
            let tidak_ada: Option<i32> = None;
            
            // Match dengan Option
            fn cek_nilai(x: Option<i32>) {
                match x {
                    Some(nilai) => println!("Nilai ada: {}", nilai),
                    None => println!("Tidak ada nilai"),
                }
            }
            
            cek_nilai(ada);
            cek_nilai(tidak_ada);
            
            // if let - cara singkat untuk match satu pola
            if let Some(nilai) = ada {
                println!("Nilai dengan if let: {}", nilai);
            }
        }
        
        // Result enum
        pub fn result_example() {
            println!("\n6. RESULT ENUM");
            println!("-------------");
            
            // Result<T, E> adalah enum bawaan Rust:
            // enum Result<T, E> {
            //     Ok(T),
            //     Err(E),
            // }
            
            // Fungsi yang mengembalikan Result
            fn bagi(a: i32, b: i32) -> Result<i32, String> {
                if b == 0 {
                    Err(String::from("Tidak bisa membagi dengan nol"))
                } else {
                    Ok(a / b)
                }
            }
            
            // Menggunakan Result dengan match
            let hasil1 = bagi(10, 2);
            let hasil2 = bagi(10, 0);
            
            match hasil1 {
                Ok(nilai) => println!("Hasil: {}", nilai),
                Err(pesan) => println!("Error: {}", pesan),
            }
            
            match hasil2 {
                Ok(nilai) => println!("Hasil: {}", nilai),
                Err(pesan) => println!("Error: {}", pesan),
            }
            
            // Cara singkat dengan unwrap (akan panic jika error)
            println!("Cara dengan unwrap (aman karena kita tahu hasilnya Ok):");
            println!("10 / 2 = {}", bagi(10, 2).unwrap());
        }

