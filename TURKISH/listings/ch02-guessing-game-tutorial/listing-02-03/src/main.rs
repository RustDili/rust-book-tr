// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Tuttuğum sayıyı tahmin edin!");

    // ANCHOR: ch07-04
    let gizli_sayı = rand::thread_rng().gen_range(1..101);
    // ANCHOR_END: ch07-04

    println!("Gizli sayı: {}", gizli_sayı);

    println!("Tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Veri okuma hatası!");

    println!("Tahmininiz:: {}", tahmin);
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
