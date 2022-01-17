// ANCHOR: here
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--
    // ANCHOR_END: here
    println!("Tuttuğum sayıyı tahmin edin!");

    let gizli_sayı = rand::thread_rng().gen_range(1..101);

    println!("Gizli sayı: {}", gizli_sayı);

    println!("Tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Veri okuma hatası!");
    // ANCHOR: here

    println!("Tahmininiz: {}", tahmin);

    match tahmin.cmp(&gizli_sayı) {
        Ordering::Less => println!("Sayınız küçük!"),
        Ordering::Greater => println!("Sayınız büyük!"),
        Ordering::Equal => println!("Kazandınız!"),
    }
}
// ANCHOR_END: here
