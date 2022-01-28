fn main() {
    let mut sayaç = 0;

    'saydır: loop {
        println!("sayaç: {}", sayaç);
        let mut kalan = 10;

        loop {
            println!("Kalan: {}", kalan);
            if kalan == 9 {
                break;
            }
            if sayaç == 2 {
                break 'saydır;
            }
            kalan -= 1;
        }
        sayaç += 1;
    }
    println!("Sayaç durdu: {}", sayaç);
}
