use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Tuttuğum sayıyı tahmin edin!");

    let gizli_sayı = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Tahmininizi girin.");

        let mut tahmin = String::new();

        io::stdin()
            .read_line(&mut tahmin)
            .expect("Satır okuma hatası!");

            let tahmin: u32 = match tahmin.trim().parse() {
            Ok(sayı) => sayı,
            Err(_) => continue,
        };

        println!("Tahmininiz: {}", tahmin);

        match tahmin.cmp(&gizli_sayı) {
            Ordering::Less => println!("Sayınız küçük!"),
            Ordering::Greater => println!("Sayınız büyük!!"),
            Ordering::Equal => {
                println!("Kazandınız!");
                break;
            }
        }
    }
}
