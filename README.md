T → mengambil ownership (move) — benar.
Contoh: fn take(s: String) {} → memindahkan String.

&T → pinjam immutable (baca saja) — benar.
Contoh: fn read(s: &String) {}

&mut T → pinjam mutable (baca + ubah) — benar.
Contoh: fn modify(s: &mut String) { s.push('!'); }

-> U (return tipe lain) → fungsi mengembalikan nilai bertipe U (ownership baru berpindah ke pemanggil).
Klarifikasi: ini bukan aturan khusus tentang “alokasi referensi baru” — hanya berarti fungsi mengembalikan sebuah nilai U (bisa dibuat baru, atau dipindahkan dari parameter).

---

Koreksi penting tentang heap / copy / move

Lokasi (heap vs stack) bukan satu-satunya penentu Copy/Move.

Sistem Copy vs Move ditentukan oleh apakah tipe mengimplementasikan trait Copy.

Banyak tipe primitif (i32, f64, bool, char, dll.) mengimplementasikan Copy → ketika kamu assign, nilai dicopy (shallow, bitwise) dan variabel lama tetap bisa dipakai.

Tipe seperti String, Vec<T>, Box<T> tidak Copy → assignment/mengirim by-value akan move ownership.

Bukan semua yang di-heap itu otomatis non-Copy.

Sebagian besar koleksi/owned types (yang punya pointer ke heap) bukan Copy (contoh umum: String, Vec).

Namun keputusan Copy tergantung implementasi trait, bukan hanya lokasi memori. (Praktisnya: tipe yg punya alokasi heap biasanya tidak Copy.)

str vs &str

str itu unsized (string slice type) — kamu tidak bisa punya let s: str = ...; langsung. Biasanya muncul sebagai &str atau Box<str>.

&str (reference ke string slice) adalah reference → references bersifat Copy (ya, &T biasanya Copy). Jadi &str dapat di-copy (copy pointer + len), tapi data string-nya mungkin di static atau di heap tergantung asalnya.

Clone vs Copy

Copy = implicit, bitwise copy, murah, no drop.

Clone = explicit, bisa mahal (allocasi), harus panggil .clone(). Banyak tipe non-Copy menyediakan .clone() untuk duplikasi eksplisit (mis. String::clone() menyalin data ke heap baru).


---

```rust
// Copy type
let a: i32 = 10;
let b = a; // copy
println!("{}", a); // masih bisa

// Move type
let s = String::from("hello");
let t = s; // move
// println!("{}", s); // ERROR: s sudah dipindahkan

// Borrow
let u = String::from("hi");
let v: &String = &u; // pinjam
println!("{}", u); // masih bisa

// &str vs str
let lit: &str = "hello"; // &'static str, data di static
let s2 = String::from("hello");
let slice: &str = &s2;   // &str yang menunjuk ke heap data milik s2
```

---
* Ringkasan
T → move (ambil ownership) — tipikal untuk owned types seperti String/Vec.

&T → borrow immutable (baca saja).

&mut T → borrow mutable (bisa ubah).

Copy vs Move ditentukan oleh trait Copy. Banyak primitif Copy. String/Vec bukan Copy.

&T (reference) biasanya Copy karena cuma pointer + metadata disalin.
