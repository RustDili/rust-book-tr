fn main() {
    let mut sayaç = 0;

    let sonuç = loop {
        sayaç += 1;

        if sayaç == 10 {
            break sayaç * 2;
        }
    };

    println!("Sonuç: {}", sonuç);
}
