// ANCHOR: here
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Tuttuğum sayıyı tahmin edin!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Gizli sayı: {}", secret_number);

    println!("Tahmininizi girin.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Veri okuma hatası!");
    // ANCHOR: here

    println!("Tahmininiz: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Sayınız küçük!"),
        Ordering::Greater => println!("Sayınız büyük!"),
        Ordering::Equal => println!("Kazandınız!"),
    }
}
// ANCHOR_END: here
