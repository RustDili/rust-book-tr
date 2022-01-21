fn main() {
    etiket_değerlerini_yazdır(5, 'h');
}

fn etiket_değerlerini_yazdır(değer: i32, birim: char) {
    println!("Etiket değerleri: {}{}", değer, birim);
}
