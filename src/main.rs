fn main() {
    // Setelah kita sekarang tahu tentang Function, kita akan bahas lagi tentang  Ownership di Function Parameter
    // Tipe data yang disimpan di Heap, ketika kita kirim sebagai parameter di function, secara otomatis Ownership nya akan berpindah ke parameter Function yang  dipanggil
    // Karena Ownershipnya berpindah ke parameter function, secara otomatis setelah function selesai dieksekusi, maka owner dan value akan dihapus dan tidak bisa digunakan lagi
    // Namun jenis data yang berada di Stack, ketika kira kirim sebagai parameter di function, maka value akan di copy

    let number = 10;
    print_number(number);

    let name = String::from("Fuad");
    hi(name);

    // return value ownership
    // Seperti yang sudah kita tahu, bahwa function bisa mengembalikan value
    // Value Heap yang kita kembalikan di function, secara otomatis ownership nya akan dimiliki oleh yang memanggil function tersebut
    // Sedangkan jika Value Stack, maka return value function akan di copy oleh yang memanggil function nya
    let first_name = String::from("Fuad");
    let last_name = String::from("Jabbar Dzakwan");
    let full_name = fullname(first_name, last_name);
    println!("My name: {}", full_name);
    // println!("first name: {}", first_name); // ini di moved ke fn fullname
    // println!("last name: {}", last_name); // sama ini juga

    // mengembalikan ownership
    // Karena kita tahu bahwa return value bisa mengembalikan ownership, jadi pada kasus jika memang kita tidak ingin mengambil ownership dari parameter, kita bisa kembalikan parameter dalam bentuk return value tuple misalnya

    let first = String::from("Fuad");
    let last = String::from("Dzakwan");
    let (first_, last_, full) = fullname_return_ownership(first, last);
    println!("{}, {}, {}", first_, last_, full);

    // Masalah Dengan Return Value Ownership
    // Jika kita tidak ingin mengambil Ownership dari parameter, maka jika tiap membuat function kita harus membuat return value tuple, maka lama-lama akan sangat menyulitkan
    // Bahkan jika dibaca, function yang kita buat akan sulit dimengerti maksudnya
    // Untungnya Rust menyediakan fitur untuk menggunakan value, tanpa harus melakukan transfer ownership, namanya adalah Reference.

    // References
    // Reference adalah pointer (penunjuk) yang bisa kita ikuti ke lokasi data aslinya di Heap. Datanya sendiri dimiliki oleh variable lain, bukan si reference
    // Reference akan dijamin menunjuk value yang valid selama alur hidup reference tersebut, jika alur hidup selesai, maka reference akan dihapus, namun tidak  dengan data yang ditunjuknya, karena data yang ditunjuk mengikuti alur hidup variable owner nya
    // Untuk membuat reference di Rust, kita bisa gunakan tanda & (and) sebelum tipe datanya, dan dalam satu waktu kita bisa membuat banyak reference
    // Sebelumnya kita tahu bahwa tipe data text str selalu kita buat dalam bentuk &str, hal ini karena defaultnya adalah reference ke str

    let nama_pertama = String::from("Bambang");
    let nama_akhir = String::from("Sutioso");
    let nama_lengkap = fullname_with_reference(&nama_pertama, &nama_akhir);
    println!("Awal: {}", nama_pertama);
    println!("Akhir: {}", nama_akhir);
    println!("Lengkap: {}", nama_lengkap);

    // Borrowing
    // Ketika kita membuat reference, aksi itu kita sebut borrowing (meminjam)
    // Kalo diibaratkan di kehidupan, kita bisa meminjam barang, tapi jika sudah selesai menggunakan barang nya, kita harus mengembalikan ke owner (pemilik) barang nya
    // Saat kita mencoba memodifikasi value dari reference, maka secara default, hal itu tidak bisa dilakukan, jadi secara default reference adalah immutable, walaupun variable owner nya sendiri adalah mutable

    // Mutable Reference
    // Pada kasus jika memang kita diperbolehkan memodifikasi value dari reference, maka kita bisa gunakan mutable reference
    // Mutable reference adalah reference dengan tanda &mut, dimana artinya kita bisa memodifikasi value dari reference tersebut
    // Namun ada ketentuan jika bisa menggunakan mutable reference, variable owner juga harus mutable, jika variable owner adalah immutable, maka mutable reference tidak bisa dilakukan
    // Selain itu, untuk menjamin keamanan, dalam satu waktu, hanya diperbolehkan membuat satu mutable reference, dan tidak ada reference lainnya

    let mut value = String::from("Bujang");
    change_name(&mut value);
    println!("Ubah value name: {}", value);

    // Dangling pointer
    // Dangling pointer adalah kondisi dimana ada reference (pointer) yang menunjuk ke value yang tidak ada di memory
    // Di Rust, hal ini tidak diperbolehkan, contoh ketika kita mengembalikan reference dalam function, maka secara otomatis value akan dihapus karena sudah keluar dari scope function
    // Pada kasus seperti ini, Rust akan menganggap hal ini error, karena berpotensi terjadi dangling pointer
    // Biasanya programmer Golang sering sekali membuat function yang mengembalikan pointer

    // Solusi dangling pointer
    // Jika menang data yang dikembalikan dibuat di dalam function, maka kita harus kembalikan dalam bentuk value langsung, bukan reference
    // Atau kita bisa mengeluarkan variable owner dari value diluar function, agar masuk variable scope, sehingga Rust tidak menghapus variable dan value tersebut setelah function selesai di eksekusi

    let awal = String::from("Awal");
    let akhir = String::from("Akhir");
    let gabung = contoh_kode_solusi_dangling_pointer(&awal, &akhir);
    println!("Digabung {}", gabung);
}

fn print_number(number: i32) {
    println!("number: {}", number);
}

fn hi(name: String) {
    println!("Hi, {}", name);
}

fn fullname(first_name: String, last_name: String) -> String {
    format!("{} {}", first_name, last_name)
}

fn fullname_return_ownership(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    (first_name, last_name, full_name)
}

fn fullname_with_reference(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

fn change_name(value: &mut String) {
    value.push_str(" New value");
}

// ini bakal ada missing lifetime, karena Rust gaboleh ada reference yang dimana lifetime itu bakal berkahir, kalo berkahir harus di buang, dan kalo dibuang memori di hapuskan.
// konotasi lifetime <'a> 'a
// fungsi ini ga akan bisa di run
// fn contoh_kode_dangling_pointer(first_name: &String, last_name: &String) -> &String {
//     let name = format!("{} {}", first_name, last_name);
//     return &name;
// }

fn contoh_kode_solusi_dangling_pointer(value1: &String, value2: &String) -> String {
    let mixed = format!("{} {}", value1, value2);
    return mixed;
}

// #[test]
// fn test_run_dangling_pointer() {
//     let awal = String::from("Awal");
//     let akhir = String::from("Akhir");
//     let gabung = contoh_kode_dangling_pointer(&awal, &akhir);
//     println!("Digabung {}", gabung);

//     // ga akan bisa di run.
// }
