// ANCHOR: all
use std::io;
// ANCHOR: ch07-04
use rand::Rng;

fn main() {
    // ANCHOR_END: ch07-04
    println!("Tuttuğum sayıyı tahmin edin!");

    // ANCHOR: ch07-04
    let secret_number = rand::thread_rng().gen_range(1..101);
    // ANCHOR_END: ch07-04

    println!("Gizli sayı: {}", secret_number);

    println!("Tahmininizi girin.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Veri okuma hatası!");

    println!("Tahmininiz:: {}", guess);
    // ANCHOR: ch07-04
}
// ANCHOR_END: ch07-04
// ANCHOR_END: all
