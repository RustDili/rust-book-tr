use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Lütfen bir dizin numarası giriniz:");

    let mut dizin = String::new();

    io::stdin()
        .read_line(&mut dizin)
        .expect("Satır okunamadı");

    let dizin: usize = dizin
        .trim()
        .parse()
        .expect("Girilen dizin numarası bir sayı olmalıdır.");

    let öğe = a[dizin];

    println!(
        "dizin {}'de bulunan öğe değeri: {}",
        dizin, öğe
    );
}
