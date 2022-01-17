use std::io;

fn main() {
    println!("Tuttuğum sayıyı tahmin edin!");

    println!("Tahmininizi girin.");

    let mut tahmin = String::new();

    io::stdin()
        .read_line(&mut tahmin)
        .expect("Veri okuma hatası!");

    println!("Tahmininiz: {}", tahmin);
}
