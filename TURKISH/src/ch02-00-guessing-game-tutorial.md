# Bir Tahmin Oyunu Programlamak

Birlikte uygulamalı bir proje üzerinde çalışarak Rust'ı kavramaya çalışalım! Bu bölümde size Rust'ın temel kavramlarından bazıları tanıtılacak ve bu kavramların gerçek bir programda nasıl kullanılacağı gösterilecektir. Bölüm boyunca `let` ve `match` anahtar kelimeleri, ilişkili metotlar ve işlevler, harici sandıklar gibi kavramlar üzerinde temel bilgilerinizi uygulayacak ve ilerleyen bölümlerde bu kavramlar ayrıntılarıyla incelenecektir.   

Projemizde klasik bir programlama problemi olan sayı tahmin oyununu kodlayacağız. Program 1 ile 100 arasında rastgele bir sayı oluşturacak ve oyuncudan bu sayıyı tahmin etmesini isteyecektir. Oyuncu tahmin ettiği sayıyı girdiğinde bu değer, programın oluşturduğu sayı ile karşılaştırılacak, sayı yüksek veya düşükse bu bilgi oyuncu ile paylaşılarak yeniden tahmin girilmesi istenecek, doğru sayı bulunduğunda bir tebrik mesajı yazdırılarak programdan çıkılacaktır.

##  Yeni Bir Proje Oluşturmak

Yeni bir proje oluşturmak için 1. Bölümde oluşturduğumuz *projeler* dizinine giderek aşağıdaki komutları uygulayın:

```console
$ cargo new tahmin_oyunu
$ cd tahmin_oyunu
```

İlk satırdaki `cargo new` komutu argüman olarak projeye verdiğimiz *tahmin_oyunu* adını alır. İkinci satırdaki `cd tahmin_oyunu` komutu bizi, Cargo tarafından oluşturulan bu yeni dizine yönlendirir. 

Cargo tarafından otomatik oluşturulan *Cargo.toml* dosyasına göz atalım:

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/Cargo.toml}}
```

Birinci bölümden hatırlayacağınız gibi `cargo new` komutu size hazır bir "Hello, world!" programı sunar. `src/main.rs` dosyasını kontrol edelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/src/main.rs}}
```

Ve bu programı `cargo run` komutu kullanarak tek seferde derleyip çalıştıralım:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-01-cargo-new/output.txt}}
```

Sıklıkla kullanılan `run` komutu, bir projeyi çabucak derleyip çalıştırmamız ve bir sonraki derleme adımına hızlıca gitmemiz gerektiğinde oldukça faydalıdır.

Programımızı oluşturacağımız *src/main.rs* dosyasını yeniden açarak kodlamaya başlayalım!

## Tahmin Verisinin İşlenmesi

Tahmin oyununun ilk bölümü, kullanıcılardan tahmin verisi olarak işleyebileceği bir değer girmesini isteyecek ve bu verinin  beklenen biçimde olup olmadığını kontrol edecektir. Oyunun başlaması için oyuncunun bir tahmin değeri girmesine izin verilecektir.  Örnek 2-1'de yer alan kodu *src/main.rs* dosyasına ekleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:all}}
```

<span class="caption"> Örnek 2-1: Bu kod kullanıcıdan tahmin verisini alarak ekrana yazdırır.</span>

Bu kod fazla bilgi içerdiğinden her satırının ayrı ayrı nceleyelim. Kullanıcı girdisini alarak sonucu çıktıta yazdırabilmek için Rust standart kütüphanesi `std`'nin bir parçası olan `io` (input/output) kütüphanesini içe aktarmamız gerekir.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:io}}
```

Standart kütüphanede tanımlanmış ve Rust'ın varsayılan olarak her program kapsamına otomatik olarak dahil ettiği bir kaç öğe vardır.


Varsayılan haliyle Rust başlatılan her program kapsamına otomatik olarak birkaç türü dahil eder. *prelude*  olarak adlandırılan bu setin içindekileri [Standart kütüphane belgelerinde][prelude] bulabilirsiniz.

Eğer kullanmak istediğiniz bir veri türü prelüd bölümünde bulunmuyorsa, bu türü `use` anahtar sözcüğü kullanarak açıkça kapsam içine almanız gerekir. Uygulamamızda kullandığımız `std::io` kütüphanesi, kullanıcı girdisini kabul etme yeteneği dahil bir dizi kullanışlı özellikle birlikte gelir.

Birinci bölümden hatırlayacağınız üzere `main()` işlevi programın giriş noktasını oluşturur.

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:main}}
```

*Function* kelimesinin kısaltılmışı olan `fn` söz dizimi yeni bir işlev bildirirken, içi boş parantezler `()` işlevin herhangi bir giriş parametresi almadığını, *açılış ayracı* olarak da bilinen sağa bakan süslü parantez `{` ise işlev gövdesinin başlangıç noktasını gösterir.

Yine 1. Bölüm'den hatırlayacağınız üzere `println!`, bir dizgiyi ekrana yazdırmak amacıyla kullandığımız bir makrodur:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print}}
```

Bu kod oyun hakkında bilgi veren ve kullanıcıdan girdi bekleyen bir komut istemi yazdırır.

### Değerleri Değişkenlerde Saklamak

Şimdi aşağıda gösterildiği gibi kullanıcı girdisini depolayacağımız bir değişken oluşturacağız:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:string}}
```

Çok şeyin gerçekleştiği bu satırda program ilginçleşmeye başlıyor. Bu satırın *değişken* oluşturmak için kullanılan bir `let` ifadesiyle başladığına dikkat edin. İşte size başka bir örnek:

```rust,ignore
let elmalar = 5;
```

Bu satır `elmalar` adında yeni bir değişken oluşturarak onu `5` değerine bağlar. Rust'ta değişkenlerin varsayılan olarak değişmez oldukları kabul edilir. Bu kavramı 3. Bölümümüz olan ["Değişkenler ve Değişkenlik"][variables-and-mutability]<!-- ignore
--> başlığı altında ayrıntılarıyla inceleyeceğiz. Bir değişkeni değiştirilebilir kılmak için  değişken adının önüne `mut` anahtar kelimesini ekleriz:

```rust,ignore
let elmalar = 5;    // değişmez
let mut muzlar = 5; // değişebilir
```

> Not: `//` söz dizimi satır sonuna kadar devam eden bir yorumu başlatır. 
> Rust'ın derleme aşamasında görmezden geldiği yorum satırlarını [3. Bölümde][comments]<!-- ignore --> tartışacağız.

Tahmin oyunumuzdaki `let mut tahmin` söz diziminin, *içeriği değiştirilebilir olarak saklanan* tahmin adında bir değişken tanımı olduğunu artık biliyorsunuz. Eşittir `=` işleciyle Rust'a, bu değişkene bir bir şeyler bağlamak istediğinizi bildirmiş olursunuz. 
Eşittir `=` işlecinin sağ tarafında, yeni bir dizgi örneği almak için kullandığımız `String::new()` işlevinden dönen ve `tahmin` değişkeninin bağlandığı değer bulunmaktadır. Dizgiler, UTF-8 baytlarıyla kodlanmış, boyutları değiştirilebilen ve standart kütüphane tarafından sağlanan [`String`][string]<!-- ignore --> türündeki metin parçalarıdır.

`String::new()` satırındaki `::` söz dizimi, `new()` işlevinin `String` türünün ilişkili işlevi olduğunu gösterir. İlişkili işlev; türe özgü, o türe ait bir uygulama olduğundan, bu durumda `new` işlevi yeni ve boş bir dizgi oluşturur. Genellikle `new` olarak adlandırılan ve ilişkili olduğu türün yeni bir değerini oluşturan bu işlevlerle Rust'ın birçok türünde karşılaşacaksınız.

Özetle `let mut tahmin = String::new();` satırında bir String türünün yeni ve boş bir örneğiyle ilklendirilen değiştirilebilir bir değişken tanımlanmaktadır.

### Kullanıcının Girdiği Veriyi Yakalamak

Hatırlayacağınız gibi programın ilk satırında `use std::io` söz dizimini kullanarak Rust standart kütüphanesinden giriş/çıkış işlevselliğini uygulamıştık. Şimdiyse `io` modülünde bulunan `stdin` işlevini çağıracağız:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:read}}
```

Eğer `io` kütüphanesini programın en başındaki `use std::io` satırınyla ithal etmemiş olsaydık, `stdin` işlev çağrısını, kod içinde `std::io::stdin` şeklinde yazarakta kullanabilirdik. `stdin` işlevi terminalinizdeki standart girdinin tanıtıcısını temsil eden bir [`std::io::Stdin`][iostdin]<!-- ignore --> tür örneği döndürür.

Sonraki `.read_line(&mut tahmin)` satırında, kullanıcıdan veri alacak olan standart girdi tanıtıcısındaki [`read_line`][read_line]<!-- ignore --> metodunu çağırılarak kendisine, girdisinin saklanacağı dizgi olan `&mut tahmin` argümanı iletilir. `read_line` metodunun bütün işi, kullanıcı tarafından girilen her veriyi standart girişe almak ve bunları bir dizgi içine yerleştirmektir.Yöntemin, kullanıcı girdisi eklendikçe dizgi içeriğini değiştirilebilmesi için, kendisine iletilen argümanın değişebilir olması gerekmektedir.

`&` belirteci, bu argümanın *referans* türünden olduğunu bildirdiğinden, kodun bazı bölümleri tarafından bu değişkenlere, bellekte defalarca kopyalanmaları gerekmeksizin erişilmesi sağlanmış olur. Referanslar dilin güçlü ve karmaşık bir özelliğidir.
Rust'ın önemli avantajlarından biri de referans kullanımının kolay ve güvenli olmasıdır. Bu programı bitirebilmeniz için daha fazla ayrıntı bilmenize gerek yok. Şimdilik tıpkı değişkenler gibi referansların da varsayılan olarak değişmez olduklarını ve onları değiştirilebilir kılabilmek için `&tahmin` yerine `&mut tahmin` yazmamız  gerektiğini öğrenmemiz yeterlidir. (Referanslar konusu 4.Bölümde ayrıntılı olarak ele alınacaktır.)

### `Result` Türünü Kullanarak Olası Hataları İşlemek

İncelememize `io::stdin` ile başlayan ifadenin üçüncü satırıyla devam edelim. Her ne kadar ayrı bir satırmış gibi görünmesine rağmen, bu satır da tıpkı bir önceki satır gibi, aynı mantıksal kod satırının parçası olup koda `expect` metodunu eklemektedir:


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:expect}}
```

Oysa bu kodu bu şekilde de yazabilirdik:

```rust,ignore
io::stdin().read_line(&mut tahmin).expect("Veri okuma hatası!");
```

Fakat böyle uzun satırları okumak zor olduğundan en iyisi onu parçalara ayırmaktır. Bir metodu .yöntem_adı() söz dizimiyle çağırdığınızda, uzun ifadeleri mantıksal parçalara bölebilmeniz için genellikle yeni satırlar ve boşluklar eklemeniz mantıklı olur. 
Şimdi bu satırın ne anlama geldiğini inceleyelim.

Daha önce bahsettiğimiz gibi `read_line` işlevi, kullanıcı tarafından girilen verileri kendisine ilettiğimiz dizgiye depolarken, bu işin gerçekleştirilmesi sırasında oluşabilecek hataların izlenebilmesi için [`io::Result`][ioresult]<!-- ignore --> türünde bir değer döndürür. Rust standart kitaplığı [`Result`][result]<!-- ignore --> olarak adlandırılan, generic türler ve `io::Result` gibi alt modüllerle kullanılmak üzere bir tür bulundurur. Varyant olarak bilinen ve sabit olasılık kümelerinden oluşan [enums][enums]<!-- ignore --> türleri genellikle eşleme işlemlerinde kullanılır. Enum kullanan eşleme işlemlerinde değerlendirilen koşul enum değerinin hangi varyantına uyuyorsa kodun o bölümü çalıştırılır. 

Hata işleme bilgilerinin kodlanmasını amaçlayan `Result` türünü 6. Bölümde ayrıntılarıyla ele alacağız.

`Result` türünün `Ok` ve `Err` adında iki varyantı bulunur. `Ok` varyantı, işlem sonucunun başarılı olması durumunda döndürülen değere ev sahipliği yaparken, işlemin başarısız olması anlamına gelen `Err` varyantında ise bu başarısızlığın nasıl ve neden olduğunu açıklayan bilgiler depolanır.

Herhangi bir türün değerleri için olduğu gibi `Result` türünün değerleri için de tanımlanmış ilişkili metodlar bulunur. Bu bağlamda`io::Result` örneğinin de [`expect`][expect]<!-- ignore --> adında bir metodu bulunmaktadır. Bu metot çağrıldığında, `io..Result` örneği `Err` değeri taşıyorsa `expect` programın çökmesine neden olacak ve kendisine argüman olarak ilettiğiniz mesajı görüntüleyecektir. `read_line` metodunun `Err` değerini döndürmesi genellikle işletim sisteminden kaynaklanan bir hatadır. Bununla birlikte `io::Result` örneği `Ok` değerini taşıyorsa, `expect` metodu `Ok` içinde saklanan dönüş değerini alarak kullanmanız için size döndürecektir. Bu durumda döndürülen `Ok` değeri kullanıcı tarafından standart girdiye iletilen bayt sayısından ibaret olacaktır.

Bu aşamada `expect` metodunu çağırmasanız bile programınız derlenir fakat aşağıdaki gibi bir uyarı alırsınız:

```console
{{#include ../listings/ch02-guessing-game-tutorial/no-listing-02-without-expect/output.txt}}
```

Rust `read_line` tarafından döndürülen `Result` değerini kullanmadığınızı ve programın olası bir hatayı işlemediğini bildirmektedir.

Her ne kadar uyarıları bastırmanın doğru yolu bir hata işleyici yazmak olsada, şu aşamada sorun oluştuğunda programın çökmesini istediğimizden `expect` metodunu kullanmak zorundayız. Hata işlemek konusunu kitabın [9. Bölümünde][recover]<!-- ignore -->.  ayrıntılarıyla inceleyeceğiz.

### `Println!` Yer Tutucuları ile Değerleri Yazdırmak

Kodun sonlandığı noktayı gösteren *kapanış ayracı* (sola bakan süslü parantez) haricinde değerlendirilmesi gereken bir satırımız daha var:


```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-01/src/main.rs:print_guess}}
```

Bu satır kullanıcı girdisini kaydettiğimiz dizgiyi ekrana yazdırabilmek için vardır. Yer tutucuları temsil eden süslü parantezleri `{}` ise bir değerin yerini tutan yengeç kıskaçlarına benzetebilirsiniz. Çok sayıda değerin gösterilmesi amacıyla da kullanabileceğiniz bu parantezlerin ilk çifti, biçimlendirilmiş dizgiden sonraki ilk değeri içerirken, sonraki parantez ikinci değeri, bir sonraki üçüncü değeri gösterecektir. İki farklı değişkenin değerlerini ekrana yazdıran örnek `println!` çağrısı aşağıdakine benzeyecektir:

```rust
let x = 5;
let y = 10;

println!("x değeri = {}, y değeri = {}", x, y);
```

Bu örnek ekrana `x değeri = 5, y değeri = 10` yazdıracaktır.

### İlk Bölümü Test Etmek

Programın ilk bölümünü test etmek için `cargo run` komutunu çalıştırın:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-01/
cargo clean
cargo run
input 6 -->

```console
$ cargo run                                                   ✔ 
   Compiling tahmin_oyunu v0.1.0 (/home/rusdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 1.34s
     Running `/home/rusdili/projeler/tahmin_oyunu/target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Tahmininizi girin.
6
Tahmininiz: 6
```

Klavyeden girdi alıp onu ekrana yazdırabildiğimize göre oyunun ilk bölümü tamamlanmış demektir. 

### Gizli Sayıyı Oluşturmak

Şimdi kullanıcının tahmin edeceği gizli sayıyı oluşturmamız gerekiyor. Oyunu eğlenceli ve tekrar oynanabilir kılabilmek amacıyla  gizli sayıyı her defasında değiştirmemiz gerekir. Oyunu kolaylaştırmak için de, tahmin edilecek sayıyı 1 ile 100 arasında ve tesadüfi biçimde seçmeliyiz. Rust'ın standart kitaplığı rastgele sayı oluşturabilecek işlevselliği henüz barındırmıyor. Ancak Rust ekibi bu işlevsellik için [`rand`][randcrate] adlı harici bir sandık sunar.

### İlave İşlevsellik İçin Sandık Kullanmak

Sandık, Rust kaynak kodu dosyalarının bir araya getirilmiş halidir. Geliştirmekte olduğumuz bu proje bile aslında bir çalıştırılabilir *ikili sandık* (binary crate) sandıktır. Bize harici olarak sunulan `rand` sandığı başka programlarda kullanılması amaçlanan kodları içeren bir *kitaplık sandığı*dır.

Harici sandıkların koordinasyonu, `Cargo` özelliklerinin ışıldadığı yerdir. `Rand` sandığı kullanan bir kod yazabilmek için önceklikle *Cargo.toml* dosyasının bu bağımlılığı içerecek şekilde güncellenmesi gerekir. Bunu gerçekleştirebilmek için aşağıdaki satırları, *Cargo.toml* dosyasında yer alan `[dependencies]` başlığının altına doğru şekilde ekleyin. Kodun sağlıklı çalışabilmesi için Rand sandığını buradaki gibi aynı sürüm numarasıyla bildirdiğinizden emin olun:

<!-- When updating the version of `rand` used, also update the version of
`rand` used in these files so they all match:
* ch07-04-bringing-paths-into-scope-with-the-use-keyword.md
* ch14-03-cargo-workspaces.md
-->

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

`Cargo.toml` dosyasındaki bölüm başlıklarının altına gelen her şey, başka bir bölüm başlayana dek o bölümün parçasıdır. Bağımlılıklar yani `[dependencies]` bölümünde Cargo'ya, projenizin çalışabilmesi için ihtiyaç duyduğu harici sandıkları ve bu sandıkların hangi sürümlerini kullanacağınızı bildirirsiniz. Bu durumda biz de projemizde kullanacağımız `rand` sandığı sürümünü `0.8.3` olarak beldireceğiz. Cargo, sürüm numaralarını bildirmekte standart olarak kullanılan [anlamsal sürümleme][semver]<!-- ignore --> sistemini -SemVer olarak da adlandırılır- yorumlamayı bildiğinden, `0.8.3`'ün aslında `^0.8.3`'ün kısaltması olduğunu anlar. Bağımlılık olarak bildirdiğimiz `rand` sandığının sürüm numarası `0.8.3`, projemizin en az `0.8.3` olan ancak `0.9.0`'ın altında kalan herhangi bir sürümle çalışabileceği anlamına gelmektedir. Bu durumda Cargo, `0.8.3`'den `0.9.0`'a kadar olan olası sandık sürümlerinin, `0.8.3` sürümüyle uyumlu genel API'ye sahip olduğunu varsayarak, projemizin derlenebilmesi için gereken en son sürümü ediner ve projemizin çalışmasını sağlar. Bununla birlikte `0.9.0` veya daha sonraki herhangi bir sürümün aşağıdaki örneklerin kullandığı API ile aynı API'ye sahip olacağı garanti edilmez.

Şimdi herhangi bir kod değişikliği yapmadan Tıpkı Örnek 2-2'de gösterildiği haliyle projeyi oluşturalım.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo clean
cargo build -->

```console
$ cargo build
  Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
  Compiling rand_core v0.6.2
  Compiling libc v0.2.86
  Compiling getrandom v0.2.2
  Compiling cfg-if v1.0.0
  Compiling ppv-lite86 v0.2.10
  Compiling rand_chacha v0.3.0
  Compiling rand v0.8.3
  Compiling tahmin_oyunu v0.1.0 (/home/rusdili/projeler/tahmin_oyunu)
   Finished dev [unoptimized + debuginfo] target(s) in 0.28s
```

<span class="caption">Örnek 2-2: Bağımlılık olarak eklenen `rand` sandığı sonrasında `cargo build` komutuyla elde edilen çıktı.</span>

Derleme esnasında oluşan çıktı işletim sisteminize bağlı olarak değişebileceğinden derlenen paket adları ve sürüm numaraları ekranınızda farklı sırayla yansıtılabilir. Bununla birlikte yüklenen sürümler *anlamsal sürümleme* sayesinde kodumuzla uyumlu olacaktır.

Harici bir bağımlılık eklediğimizde Cargo, [Crates.io][cratesio]'daki verilerin bir kopyası olan *kayıt defteri*nden, ihtiyaç duyduğumuz tüm bağımlılıkların en son sürümlerini çekecektir. Crates.io, Rust ekosistemindeki geliştiricilerin açık kaynak projelerini başkaları ile paylaşmak amacıyla sandıklar şeklinde yayınladıkları çevrimiçi bir kaynaktır.   

Kayıt defteri güncellendikten sonra Cargo, `[dependencies]` bölümünü kontrol ederek henüz sahip olmadığımız sandıkları indirir. Bağımlılık olarak yalnızca `rand` kütüphanesi eklense bile, Cargo bu kütüphanenin çalışabilmesi için gerekli diğer sandıkları da indirecektir. Gerekli sandıklar indirildikten sonra Rust önce bu sandıkları derleyecek, arkasından projemizi mevcut bağımlılıklar ile yeniden oluşturacaktır.  

Herhangi bir değişiklik yapmadan `cargo build` komutunu yeniden çalıştırırsanız, uçbiriminizde `Finished` satırınndan başka çıktı alamazsınız. Bu eylemsizlik Cargo'nun; bağımlılıkların indirilip derlendiğini, kodda değişiklik yapılmadığını ve *Cargo.toml* dosyasının aynı kaldığını bilmesinden kaynaklanır. Bu durumda yapacak bir şey olmadığını fark eden Cargo programı derlemeden süreci sonlandırır.

Fakat *src/main.rs* dosyasını açıp üzerinde basit bir değişiklik yaparak kaydedip derlerseniz, yalnızca iki satırdan oluşan aşağıdaki çıktıyla karşılaşırsınız: 

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
touch src/main.rs
cargo build -->

```console
$ cargo build
   Compiling tahmin_oyunu v0.1.0 (/home/rusdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
```

Bu satırlar derlemenin sadece *src/main.rs* dosyasındaki küçük değişiklikler gözetilerek gerçekleştirdildiğini gösterir. Bağımlılıkların değişmediğini ve projenin, önceden indirilip derlenen bağımlılıklarla kullanılmasının mümkün olduğunu anlayan Cargo, kodu sadece değişen kısmıyla yeniden oluşturur.

#### `Cargo.lock` Dosyası ile Yinelenebilir Derlemeler

Cargo, siz veya başkaları tarafından kodunuzun her derlenişinde aynı yapıyı yeniden oluşturan bir mekanizmaya sahiptir. Bu Cargo'nun siz aksini söyleyene kadar sadece bildirdiğiniz bağımlılık ve sürümlerini kullanması anlamına gelir. Örneğin `rand` sandığının yeni sürümü 0.8.4'ün önemli bir hata düzeltmesiyle yakın bir zamanda yeniden yayınlanacağını varsayalım. Bu durumda ne olacağının yanıtı, `cargo build` komutunu ilk çalıştırdığınızda *tahmin_oyunu* dizininde oluşturulan *Cargo.lock* dosyasında bulunmaktadır.

Bir projeyi ilk kez derlediğinizde kriterlere uyan tüm bağımlılık sürümleri Cargo tarafından belirlenerek *Cargo.lock* dosyasına yazılır. Daha sonra projenin yeniden derlemmesi gerektiğinde Cargo, *Cargo.lock* dosyasının halihazırda var olduğunu görecek ve tüm sürüm oluşturma işlemlerini yapmak yerine, orada belirtilmiş sürümleri kullanacaktır. Bu sizin otomatik olarak tekrarlanabilir derlemelere sahip olmanızı sağlar. Başka bir ifadeyle, *Cargo.lock* dosyası sayesinde projeniz siz yeniden ve açıkça yükseltme yapmadığınız sürece `0.8.3` sürümünde kalmaya devam eder.

#### Bir Sandığı Yeni Bir Sürüme Güncellemek

Bir sandığı güncellemek istediğinizde Cargo size, *Cargo.lock* dosyasını yok sayacak ve *Cargo.toml* dosyanızdaki kriterlere uygun son sürümleri bulmanızı sağlayacak `update` adında bir komut daha sağlar. Süreç başarıyla tamamlanırsa güncellenen bu sürümler *Cargo.lock* dosyasına yazılır. Ancak güncelleme esnasında varsayılan olarak sadece `0.8.3`'ten büyük `0.9.0`'dan küçük olan sürümler aranacaktır. Eğer `rand` sandığı için `0.8.4` ve `0.9.0` olmak üzere iki yeni sürüm yayınlanmışsa `update` komutunu çalıştırdığınızda aşağıdaki gibi bir çıktı görünecektir:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-02/
cargo update
Cargo'nun yeni sürümümnün 0.8.x olduğunu varsaydığımızda
Diğer türlü burada sunulan varsayımsal çıktıyı elde edebilmek için
Rehber olarak başka bir güncellemeden yararlanın -->

```console
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Bu noktada *Cargo.lock* dosyanızda kullanmakta olduğunuz `rand` sandığı sürümünün, 0.9.0 sürümünün yok sayılarak `0.8.4`'e yükseltildiğini belirten değişikliğin yapıldığını fark edeceksiniz. Eğer rand sandığının `0.9.0` veya `0.9.x` sürümlerinden birini kullanmak isterseniz, *Cargo.toml* dosyanızı aşağıdaki şekilde güncellemeniz gerekir:

```toml
[dependencies]
rand = "0.9.0"
```

`cargo build` komutunu yeniden çalıştırdığınızda, Cargo mevcut sandıkların kayıtlarını güncelleyerek `rand` kütüphanesi gereksinimlerini bildirdiğiniz yeni sürüme göre yeniden değerlendirecektir.

[Cargo][doccargo]<!-- ignore --> ve [Ekosistemi][doccratesio]<!-- ignore --> hakkında söylenecek çok şey olmasına rağmen bunları, 14. Bölümde enine boyuna tartışacağız. Şimdilik Cargo'nun, kitaplıkların yeniden kullanımını kolaylaştırarak geliştiricilerin, bir dizi paketten oluşan küçük projeler yazabilmelerini sağladığını bilmemiz yeterlidir. 

### Rastgele Sayının Üretilmesi

Artık rastgele sayıyı üretebilmek için `rand` sandığını kullanabiliriz. Yapacağımız ilk şey *src/main.rs* dosyamızı örnek 2-3'te olduğu gibi güncellemektir.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:all}}
```

<span class="caption">Örnek 2-3: Rastgele sayı üretmek için eklenen kodlar.</span>

Önce projemizin kapsam alanına `use rand::Rng` şeklinde bir `use` satırı ekliyoruz. Rand kitaplığının `Rng` özelliği, rastgele sayı üreteçlerinin uyguladığı metotları tanımladığından, bu yöntemin kullanabilmesi için kütüphanenin kapsama dahil edilmesi gerekir. Özellikler *(trait)* konusuna 10. Bölümde değineceğiz.

Ardından ilk ekran çıktısını üreten satırdan sonra iki satır daha ekleyeceğiz. Bu satırlardan ilki olan `rand::thread_rng()` işlevinde, işletim sistemi tarafından başlatılan ve geçerli olan iş parçacığına özgü kullanılan rastgele sayı üreteci başlatılacak ve üretilecek olan sayı `ı` adlı değişkende saklanacaktır. Bu sayının üretiminde ise `rand::Rng` olarak kapsama alanına dahil ettiğimiz `Rng` özelliğinde tanımlanmış `gen_range()` metodundan yararlanılacaktır. Kendisine verilen bir aralığa göre rasgele sayı üreten `gen_range()` metodunda kullanılan aralık ifadesi `başlangıç..bitiş` şeklinde olup, başlangıç olarak verilen alt sınır değeri kapsanmakta, bitiş olarak verilen üst sınır değeri ise hariç tutulmaktadır. Bu nedenle 1 ile 100 arasındaki sayılar arasından birini rastgele olarak talep edebilmemiz için metoda ileteceğimiz aralık değerlerini, aralığa dahil edilecek olan 1 ile aralığa dahil edilmeyecek olan üst sayı sınırını bildiren 101 olarak bildirmemiz gerekir. Eğer bu ifade biçimi size karışık geliyorsa, aynı işi yapan ve hem başlangıç hem de bitiş değerlerini aralığa dahil olarak gösterebileceğiniz `1..=100` şeklindeki gösterimi `gen_range()` metoduna aralık olarak iletebilirsiniz.

> Bir sandığın hangi özellik, metot ve işlevlerinin kullanılabileceğini her zaman bilemeyebilirsiniz.
> Sandıkların nasıl kullanılması gerektiğine dair talimatlar o sandığa ait belgelerde yer almaktadır.
> Cargo'nun bir başka güzel özelliği de, tüm bağımlılıklarınız tarafından sağlanan dökümantasyonu yerel 
> olarak oluşturup, tarayıcınızda uyumlu olarak çalıştıracak olan `cargo doc --open` komutunu sağlamasıdır.
> örneğin `rand` sandığındaki bulunan diğer işlevler hakkında bilgilenmek istiyorsanız, `cargo doc --open`
> komutunu çalıştırarak, sol kenar çubuğunda yer alan `rand` seçeneğine tıklamanız yeterlidir.     

Eklediğimiz ikinci satır ise `gizli_sayı` değişkenini yazdırmak için kullanılacaktır. Kodumuzun gelişme aşamasında test amaçlı kullanacağımız bu satır, programımızın nihai sürümünde yer almayacaktır. Başlatılır başlatılmaz gizli kalması gereken sayıyı açık eden program oyun değildir!

Programı birkaç defa çalıştırarak deneyin:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-03/
cargo run
4
cargo run
5
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (/home/rusdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
     Running `/home/rusdili/projeler/tahmin_oyunu/target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 73
Tahmininizi girin.
11
Tahmininiz:: 11

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/rusdili/projeler/tahmin_oyunu/target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 69
Tahmininizi girin.
88
Tahmininiz:: 88
```

Program her çalıştırıldığında 1 ile 100 arasında tesadüfi bir sayı göstermelidir. Güzel iş!

## Tahmin Sayısının Gizli Sayı ile Karşılaştırılması

Elimizde kullanıcıdan alınan bir tahmin sayısı ve tasadüfi olarak üretilen bir `gizli_sayı` olduğuna göre bunları karşılaştırabiliriz. Kodun bu bölümü Örnek 2-4'te gösterilmekle beraber, henüz açıklayacağımız nedenlerden ötürü derlenmez. 
<!-- Kaldım -->
<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-04/src/main.rs:here}}
```

<span class="caption">Örnek 2-4: İki sayıyı karşılaştırarak olası dönüş değerlerini işlemek.</span>

Buradaki ilk yenilik standart kitaplıktaki, `std::cmp::Ordering;` türünün yeni bir `use` deyimi kullanılarak kod kapsamına getirilmiş olmasıdır. `Result` türü gibi bir `enum` olan `Ordering` türünün `less`, `Greater`, `Equal` şeklinde üç karşılaştırma varyantı vardır ve bunlar, iki değeri karşılaştırırken ortaya çıkan üç olası sonucu temsil etmekte kullanılırlar.

Koda eklenen ikinci yenilik ise, `Ordering` türünü kullanmak amacıyla kodun en alt kısmına yerleştirdiğimiz beş yeni satır içeren bir eşleme ifadesidir. İfadenin kullandığı `cmp` metoduysa bir karşılaştırma işlevidir ve burada iki değerin karşılaştırılması amacıyla kullanılır. Karşılaştırılması istenen değerin referansını alarak çalışan bu metot, `tahmin` değişkeni içindeki değeri `gizli_sayı` değişkenindeki değer ile karşılaştıracak ve `use` anahtar kelimesiyle kod kapsamına aldığımız `Ordering` türünün varyantlarından uygun olan birini döndürecektir. Elde edilen dönüş değeriyle ne yapılacağına ise `tahmin` ve `gizli_sayı` değerlerini karşılaştıran `cmp` çağrısından döndürülecek olası sonuçlarla eşleştirilen ifadelerle karar verilecektir. 

Dilimize *eşleme* olarak çevirebileceğimiz [`match`][match]<!-- ignore --> olası durumları ifade eden dallardan meydana gelir. Bu dallar, bir örüntü *(kalıp, şablon)* ve eşleme ifadesinin başlangıcında belirtilen değerin bu örüntüyle eşleşmesi halinde yürütülecek olan koddan ibarettir. Eşleştirilecek değeri alan Rust bunu sırasıyla her dalın örüntüsüyle karşılaştıracak ve eşleşen daldaki kodu işletecektir. Rust'ın `match` yapısı ve örüntüleri, kodunuzda karşılaşabileceğiniz çeşitli durumları ifade etmenize yarayan ve olası her durumun ele alındığından emin olmanızı sağlayan güçlü özelliklerdir. Bu özellikler sırasıyla 6. ve 18. bölümlerde ayrıntılı biçimde ele alınacaktır.

Burada kullanılan eşleme ifadesinin nasıl çalışacağını anlayabilmek için kullanıcının tahmin ettiği sayının 50, rasgele üretilen sayının da 38 olduğunu varsayalım. Kod 50 ile 38 sayılarını karşılaştırdığında, 50 sayısı 38'den büyük olduğundan `cmp` metodu `Ordering::Greater` döndürecek ve `match` ifadesi `Ordering::Greater` değerini alarak her dalın örüntüsünü teker teker kontrol etmeye başlayacaktır. İlk dalın `Ordering::Less` örüntüsü kontrol edildiğinde, bu değerin `Ordering::Greater` ile eşleşmediği görülecek ve bu daldaki kodlar yok sayılarak hemen bir sonraki dala geçilecektir. Geçilen bu dal incelendiğinde, daldaki `Ordering::Greater` örüntüsünün `match` ifademizin almış olduğu `Ordering::Greater` değeriyle aynı olduğu görülecek ve bu koldaki kodlar çalıştırılarak ekrana `Sayınız büyük!` mesajı yazdırılacaktır. Artık bir eşleme bulunmuş olduğundan `match` ifadesi kalan son dala bakmaya gerek duymayacak ve çalışmasını sonlandıracaktır.

Ancak Örnek 2-4'ü çalıştırdığımızda henüz derlenmediğini görürüz:

```console
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-04/output.txt}}
```

Çıktıda sorunun *tür uyumsuzluğundan* kaynaklandığı belirtiliyor. Rust güçlü ve statik tür sistemiyle birlikte türün bağlamdan çıkarsanması özelliğine de sahip bir programlama dili olduğundan, tahmin değişkenini `let mut tahmin = String::new()` olarak bildirdiğimizde, değişkenin `String` türünde olacağını varsayar. Fakat programın rastgele ürettiği `gizli_sayı` ise sayı türüdür. Rust'ta 1 ile 100 arasındaki sayıları gösterebilecek belli başlı sayısal türler vardır. Bunlar, işaretli 32 bitlik sayılar için `i32`, işaretsiz 32 bitlik sayılar için `u32`, işaretli 64 bitlik sayılar için kullanılan `i64` gibi türleridir. Rust tamsayılar için varsayılan olarak `i32` türünü benimsediğinden, tür bilgisi kodun herhangi bir yerinde açıkça belirtilmedikçe `i32` olarak varsayılacak, `gizli_sayı` değişkeni `i32` olarak atanacaktır. Bu durumda bir `String` türüyle `i32` türü karşılaştırılamayacağından Rust, tam olarak karşılaştığımız hatayı üretecektir.

Bu sorunu çözebilmemiz için, kullanıcı girdisi olarak okunan `String` türünü gerçek bir sayı türüne dönüştürüp, sayısal değerli `gizli_sayı` değişkeniyle karşılaştırmamız gerekir. Bunu `main()` işlevine ekleyeğimiz tek satır kod ile gerçekleştirebiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/src/main.rs:here}}
```

Eklenen yeni satır:

```rust
let tahmin: u32 = tahmin.trim().parse().expect("Lütfen bir sayı türü girin!");
```

Bu satır `tahmin` adında yeni bir değişken oluşturur. Hatırlarsanız programımızda kullanılan bir `tahmin` değişkeni zaten vardı. O halde bu satırda yeniden oluşturulan `tahmin` değişkenin anlamı nedir? Rust bir değişkeni, aynı adlı başka bir değişkenle değiştirmemize izin verir. Gölgeleme olarak adlandırılan bu özellik, bir değeri olduğu türden başka bir türe çevirmek istediğiniz durumlarda oldukça kullanışlıdır. Bu özellik örneğin `tahmin` ve `bir_başka_tahmin` gibi iki farklı değişken oluşturmak yerine `tahmin` değişken adını tekrar kullanmamıza izin verir. Sıklıkla bir türün başka bir türe dönüştürülmesinde kullanılan  Gölgeleme olanağını 3. Bölümde tartışacağız.

Yeni`tahmin` değişkenini `tahmin.trim().parse()` ifadesine bağladığımızda, ifade içindeki `tahmin`, `String` türündeki kullanıcı girdisini içeren orjinal `tahmin` değişkenini gösterir. Bir `String` örneğine uygulanan `trim` metodu ise kendisine iletilen dizginin baş ve sonunda bulunan beyaz boşlukları temizler. Her ne kadar `u32` türü yalnızca sayısal karakterler içeriyor olsa da, kullanıcının `read_line` işlemini yerine getirmek için enter tuşuna basmasıyla dizgiye yeni bir satır eklenecektir. Örneğin, kullanıcı tahmini ettiği 5 rakamını yazıp enter tuşuna bastığında, `tahmin` içindeki veri `5\n` olarak görünecektir. Bu, kullanıcının girdiği rakama İngilizce karşılığı "newline" olan ve *yeni bir satırı* temsil eden `\n` karakterinin eklenmesi anlamına gelir. `trim` metodunun kullanılması, `\n` karakterinin temizlenerek girdinin sadece 5 olarak kalmasını sağlar. 

Dizgilerle kullanılan [`parse`][parse]<!-- ignore --> metodu ise, dizgiyi sayı türüne ayrıştırır. Bu metot çeşitli sayı türlerini ayrıştırabildiğinden, istenilen sayı türünün Rust'a tam olarak `let tahmin: u32 ` şeklinde açıkça bildirilmesi gerekir. `tahmin` değişkeninden sonra gelen `(:)` iki nokta ise, bildirilen değişkene tür açıklaması ekleneceğini gösterir. Rust'ta birkaç yerleşik sayısal tür bulunur ve burada kullandığımız `u32` türü, işaretsiz 32 bitlik bir tamsayıyı olduğundan, küçük bir pozitif sayı için uygun bir seçimdir (Diğer sayı türlerini 3. Bölümde inceleyeceğiz.).  `tahmin` değişkenine `u32` olarak eklenen tür açıklaması ve `tahmin` değişkeninin `gizli_sayı` ile karşılaştırılması sayesinde Rust, bu bağlamdan `gizli_sayı` değişken türünün `u32` olacağını çıkarır. Artık karşılaştırma işlemi, aynı türden iki değer arasında gerçekleştirilecektir!

Dizgi içeriğinde `A👍%` şeklinde bir değerin bulunması halinde, bu değeri bir sayıya sorunsuzca dönüştürmenin herhangi bir yolu olmadığından, `parse` çağrısı kolaylıkla bir hata üretebilir. Bu nedenle `parse` metodu, başarısız olma ihtimaline karşı daha önce [*`Result` Türü ile Olası Hataları İşlemek*](#result-türü-ile-olası-hataları-i̇şlemek)<!-- ignore --> başlığında incelediğimiz gibi ve `read_line` metoduna benzer şekilde bir `Result` türü döndürür. Döndürülen `Result` türünü ise `expect` metodunu kullanarak değerlendireceğiz. Eğer `parse` metoduyla dizgiden bir sayı elde edilemez ve `Result` türü `Err` varyantını döndürürse `expect` çağrısı programı çökertecek ve kendisine parametre olarak ilettiğimiz *Lütfen bir sayı türü girin!* mesajını gösterecektir. Fakat `parse` metodu başarılı olur ve bir sayı üretebilirse, `Result` türü `Ok` varyantını döndüreceğinden `expect` çağrısından da `Ok` varyantı içinde depolanan bu değer döndürülmüş olacaktır.    

Şimdi programımız yeniden çalıştıralım!
<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-03-convert-string-to-number/
cargo run
  76
-->

```console
$ ccargo run                                          ✔  5s  
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 14
Tahmininizi girin.
76
Tahmininiz: 76
Sayınız büyük!

```

Kullanıcı girdisiyle alınan 76 sayısının önünde boşluklar olmasına rağmen kodun, tahmin değerini 76 olarak alınabiliyor olması güzel! Lütfen programınızı "Sayınız küçük!", "Sayınız büyük!" ve "Bildiniz!" seçeneklerini üretecek şekilde birkaç defa çalıştırarak gözlemleyin.

Oyunun büyük bölümü doğru çalışıyor olsa da kullanıcıların yalnızca bir tahmin hakkı olması bütün eğlenceyi bozuyor. Koda bir döngü ekleyerek bu durumu değiştirebiliriz!

## Döngü Kullanarak Farklı Tahminler Almak

Bir anahtar kelime olan `loop` sonsuz döngü oluşturur. Kullanıcıların doğru sayıya ulaşmalarını kolaylaştırmak amacıyla programımıza `loop` döngüsü ekleyecek ve onlara daha fazla şans vereceğiz.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-04-looping/src/main.rs:here}}
```

Göreceğiniz gibi 'tahmin giriş talebi'nden itibaren olan her şeyi döngü kapsamına taşıyarak, her satır için dört boşluk değerinde girinti oluşturduk. Programı çalıştırdığınızda kodun tam olarak istediğimiz şeyi yapmakla beraber, sonsuza kadar tahmin yapılmasını bekleyen yeni bir sorunun oluştuğunu ve kullanıcıların bu döngüden çıkmasının engellediğini fark edeceksiniz!

Kullanıcılar *ctrl+d* klavye kısa yolunu kullanarak programı her zaman sonlandırabilirler. Ancak bu doyumsuz canavardan kaçmanın başka bir yolu daha var. Hatırlarsanız [Tahmin Sayısının Gizli Sayı ile Karşılaştırılması](#tahmin-sayısının-gizli-sayı-ile-karşılaştırılması)<!-- ignore --> başlığındaki `parse` konusundan tartıştığımız gibi, tahmin verisine sayısal olmayan bir değer verilmesiyle programın çökerek sonlanıyordu. O haldei kullanıcıların döngüyü kırarak programdan çıkmalarını sağlamak için bundan yararlanabiliriz.

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/no-listing-04-looping/
cargo run
(too small guess)
(too big guess)
(correct guess)
quit
-->

```console
$ cargo run 
   Compiling tahmin_oyunu v0.1.0 (/home/rusdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 26
Tahmininizi girin.
45
Tahmininiz: 45
Sayınız büyük!
Tahmininizi girin.
11
Tahmininiz: 11
Sayınız küçük!
Tahmininizi girin.
30
Tahmininiz: 30
Sayınız büyük!
Tahmininizi girin.
çıkış
thread 'main' panicked at 'Lütfen bir sayı türü girin!: ParseIntError { kind: InvalidDigit }', src/main.rs:178:49
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

```

Tahmin değişkenine *çıkış* gibi sayısal olmayan herhangi bir ifadenin girilmesi programdan çıkılmasına yetiyor gibi görünse de bu mekanizma, "Tahmin sayısının doğru girilmesi halinde programın otomatik olarak sonlanması" talebimizi henüz karşılamıyor.

### Doğru Tahmin Sonrası Oyundan Çıkmak

Kullanıcının doğru tahmin yaparak oyunu kazanması durumunda, programdan çıkılmasını sağlayan `break` anahtar kelimesini kodlarımıza ekleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/no-listing-05-quitting/src/main.rs:here}}
```

Kullanıcın doğru tahmini yaptığı ve "Bildiniz!" mesajının ekrana yazdırıldığı satırın ardına eklenen `break` ifadesi programın döngüden çıkmasını sağlar. Döngü `main` işlevinin son bölümü olduğundan döngüden çıkmak aynı zamanda programdan çıkmak anlamına da gelir.

### Geçersiz Veri Girişlerini İşlemek

Oyunun davranışını daha da iyileştirebilmek amacıyla, sayısal olmayan bir değer alındığında programı çökertmek yerine, bu değerlerin yok sayılmasını ve kullanıcının doğru sayıyı bulana kadar tahmine devam etmesini sağlayalım. Bu iyileştirmeyi Örnek 2-5'te gösterildiği şekilde, `String` türündeki `tahmin` değişkenini, `u32` türüne dönüştüren satırda değişiklik yaparak gerçekleştirebiliriz.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-05/src/main.rs:here}}
```

<span class="caption">Örnek 2-5: Sayı olmayan veriyle programı çökertmek yerine yeni bir tahmin istemek</span>

`expect` çağrısının `match` ifadesiyle değiştirilmesi, programı çökerten hatadan düzgün şekilde işlenen hataya geçilmesini sağlar. Ayrıştırma işlemini gerçekleştiren `parse` metodunun bir `Result` türü döndürdüğünü ve bu türün `OK` veya `Err` varyantlarına sahip bir `enum` türü olduğunu unutmayın. Tıpkı `cmp` metodunun `Ordering` türünden döndürdüğü sonuçları işlediğimiz gibi burada da bir `match` ifadesi kullandıyoruz.

`parse` metodu dizgiyi bir sayıya düzgünce dönüştürebilirse, elde edilen sayıyı içeren bir `Ok` değeri döndürülür. Bu değer ilk dalın örüntüsüyle eşleştiğinde `match` ifadesi, `parse` ile oluşturulan `sayi` değerini alarak `Ok` değerinini içine yerleştirecek ve bu sayı yeni oluşturulan `tahmin` değişkeninde saklanacaktır.

Dizgi sayıya dönüştürülemiyorsa da, hata hakkında detaylı bilgi içeren `Err` değeri döndürülücektir. Bu değer `match` ifadesinin `Ok(sayi)` dalıylae değil, ikinci daldaki `Err(_)` kalıbıyla eşleşecektir. Bu kalıpta yer alan alt çizgi `_` bize, içindeki değerlere bakılmaksızın `Err` varyantındaki tüm değerlerin bu dal ile eşleştirileceğini söylemektedir. Burası çalıştığında, döngünün bir sonraki yinelemesine atlanarak yeni bir tahmin verisi istemesini sağlayan `continue` ifadesi işletilecek, böylece `parse` metodunun karşılaşabileceği olası tüm hatalar yok sayılmış olacaktır.       

Bu aşamada artık programımızdaki her şey beklendiği gibi çalışacaktır. Deneyelim:

<!-- manual-regeneration
cd listings/ch02-guessing-game-tutorial/listing-02-05/
cargo run
(Küçük sayı tahmini)
(büyük sayı tahmini)
çıkış
(doğru tahmin)
-->

```console
$ cargo run
   Compiling tahmin_oyunu v0.1.0 (/home/rusdili/projeler/tahmin_oyunu)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/tahmin_oyunu`
Tuttuğum sayıyı tahmin edin!
Gizli sayı: 63
Tahmininizi girin.
20
Tahmininiz: 20
Sayınız küçük!
Tahmininizi girin.
99
Tahmininiz: 99
Sayınız büyük!
Tahmininizi girin.
çıkış
Tahmininizi girin.
63
Tahmininiz: 63
Kazandınız!

```

Mükemmel! Küçük ve son bir ince ayar daha yaptıktan sonra oyunu bitireceğiz. Test aşamasında gizli sayının ekrana yazdırılması önemli bir detayken, bunun sonuç aşamasında halen var olması oyunu mahvediyor. Bu durumu `gizli_sayı` değişkenini ekrana yazdıran `println!` satırını silerek düzeltelim. Örnek 2-6 kodun son halini gösterir.

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-06/src/main.rs}}
```

<span class="caption">Örnek 2-6: Tahmin oyunu programının son hali</span>

## Özet

Tebrikler! Başarıyla çalışan bir sayı tahmin oyunu programladınız.

Bu proje, `let`, `match`, *metotlar*, *ilişkili işlevler*, harici sandıkların kullanılması gibi birçok Rust kavramını size tanıtmanın uygulamalı bir yoluydu. Kitabın ilerleyen bölümlerinde bu kavramlar hakkında daha çok şey öğreneceksiniz. 
3. Bölümde değişkenler, veri türleri, işlevler gibi çoğu programlama dili tarafından kullanılan kavramları kapsanacak ve bunların Rust ile nasıl kullanıldığı gösterilecektir. 4. Bölümde ise Rust'ı diğer dillerden ayıran önemli bir özellik olan mülkiyet kavramı incelenecek, 5. Bölümde yapı ve metot söz dizimleri tartışılacak, 6. bölümdeyse `enum` türünün çalışması irdelenecektir.
<!-- 
std bağlantıları kontrol edildi
örnek kontrolü yapılacak
-->
[prelude]: https://doc.rust-lang.org/std/prelude/index.html
[variables-and-mutability]: ch03-01-variables-and-mutability.html#degiskenler-ve-degiskenlik
[comments]: ch03-04-comments.html
[string]: https://doc.rust-lang.org/std/string/struct.String.html
[iostdin]: https://doc.rust-lang.org/std/io/struct.Stdin.html
[read_line]: https://doc.rust-lang.org/std/io/struct.Stdin.html#method.read_line
[ioresult]: https://doc.rust-lang.org/std/io/type.Result.html
[result]: https://doc.rust-lang.org/std/result/enum.Result.html
[enums]: ch06-00-enums.html
[expect]: https://doc.rust-lang.org/std/result/enum.Result.html#method.expect
[recover]: ch09-02-recoverable-errors-with-result.html
[randcrate]: https://crates.io/crates/rand
[semver]: http://semver.org
[cratesio]: https://crates.io/
[doccargo]: http://doc.crates.io
[doccratesio]: http://doc.crates.io/crates-io.html
[match]: ch06-02-match.html
[parse]: https://doc.rust-lang.org/std/primitive.str.html#method.parse
