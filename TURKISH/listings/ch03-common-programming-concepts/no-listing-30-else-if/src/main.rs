fn main() {
    let sayı = 6;

    if sayı % 4 == 0 {
        println!("Sayı 4' e kalansız bölünebilir.");
    } else if sayı % 3 == 0 {
        println!("Sayı 3' e kalansız bölünebilir.");
    } else if sayı % 2 == 0 {
        println!("Sayı 2' ye kalansız bölünebilir.");
    } else {
        println!("Sayı 4, 3 veya 2'ye kalansız bölünemez!");
    }
}
