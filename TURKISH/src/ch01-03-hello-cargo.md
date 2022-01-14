## Merhaba, Cargo

Cargo Rust'ın derleme sistemi ve paket yöneticisidir. Bu araç sizin için; kod oluşturmak, kodun bağımlı olduğu kütüphaneleri *kodunuzun ihtiyaç duyduğu kitaplıklara bağımlılık adını veriyoruz)* indirmek ve bunları derlemek gibi pek çok görevi yerine getirdiğinden çoğu Rustacean bu aracı Rust projelerini yönetmek için kullanır.

Şu ana kadar yazdığımıza benzeyen basit Rust programları herhangi bir kütüphaneye bağımlı değildir. Bu nedenle "Merhaba dünya!" gibi basit bir proje Cargo ile derlendiğinde, sadece Cargo'nun aracının kod derlemeyi yöneten bölümü kullanılır. Yazılan programlar basitten karmaşığa doğru evrildikçe farklı kütüphanelere olan bağımlılıkları artacağından Cargo aracı bu bağımlılıkların yönetilmesinde size büyük kolaylıklar sağlayacaktır.

Rust projelerinin büyük çoğunluğu Cargo aracını kullandığından, bu kitabın geri kalan bölümlerinde sizin de bu aracı kullandığınız varsayılacaktır. Eğer ["Kurulum"](ch01-01-installation.md) bölümünde önerilen resmi yükleyicileri kullandıysanız Rust'la birlikte Cargo aracınız da yüklenmiş olmalıdır. Ancak Rust'ı farklı bir yoldan kurduysanız aşağıdaki kodları terminalinize girerek Cargo'nun sisteminizde kurulu olup olmadığını öğrenebilirsiniz.

```console
$ cargo --version
```

Çıktınızda bir sürüm numarası görüyorsanız bu, Cargo aracının Rust kurulumuyla birlikte yüklendiği anlamına gelir. Eğer `Command not found` gibi bir hatayla karşılaşıyorsanız Cargo'nun nasıl kurulacağına dair kullandığınız kurulum yöntemi belgelerini incelemelisiniz.

### Cargo ile Proje Oluşturmak

Cargo aracını kullanarak yeni bir proje oluşturup, bu yeni projenin önceki projemiz olan "Merhaba dünya!" ile farklılıklarını incelemeye çalışalım. Başlangıçta açtığımız *projeler* dizini -ya da kodlarınızı nereye kaydediyorsanız- o dizine geçerek kullandığınız işletim sisteminden bağımsız olarak iş gören aşağıdaki komutları çalıştırın:

```console
$ cargo new merhaba_cargo
$ cd merhaba_cargo
```

İlk komut, Cargo aracının "merhaba_cargo" adlı yeni bir dizin açmasını ve bu dizinde bir proje için gerekli olan dosyaları oluşturmasını sağlar. 

Sonraki komut, Cargo aracının bizim için oluşturduğu dizine atlamamızı sağlar. Dizindeki dosyaları listelerseniz Cargo aracıyla oluşturulan *Cargo.toml* dosyası ve içinde *main.rs* dosyasını bulunduran bir *src* dizini göreceksiniz.

Ek olarak Cargo aracı .gitignore dosyasını içeren yeni bir git deposunun da başlatılmasını sağlar. Eğer `cargo new` komutunu halihazırda mevcut olan bir git deposu içinde çalıştırırsanız bu git dosyaları oluşturulmaz. Bu davranışı `cargo new --vcs=git` komutunu kullanarak baskılayabilirsiniz.

> Not: Git yaygın olarak kullanılan bir sürüm kontrol sistemidir. Cargo'yu --vcs bayrağı aracılığıyla farklı bir 
> sürüm kontrol sistemi kullanmak ya da sürüm kontrol sistemini kullanmamak üzere ayarlayabilirsiniz. Mevcut 
> seçenekleri görmek için `cargo new -help` komutunu çalıştırabilirsiniz.

Cargo.toml dosyasını metin düzenleyicinizde açtığınızda içeriği Örnek 1-2'dekine benzer biçimde görünmelidir.

<span class="filename">Dosya adı: Cargo.toml</span>

```toml
[package]
name = "merhaba_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```

<span class="caption"> Örnek 1-2: `cargo new` komutuyla oluşturulan Cargo.toml dosyası içeriği</span>

Bu dosya, Cargo'nun yapılandırma formatı olan [TOML](https://toml.io/en/)(Tom's Obvious, Minimal Language) biçimindedir.

İlk satırda bildirilen ve altındaki ifadeler tarafından oluşturulan [package] bölüm başlığı, paketin nasıl yapılandırıldığını gösterir. Bu dosyaya daha fazla bilgi ekledikçe, başka bölümler de ekleyeceğiz. 

Sonraki üç satır, programınızın Cargo tarafından derlenebilmesi için gereken: İsim, programınızın sürüm bilgisi ve Rust sürümü gibi yapılandırma bilgilerinden oluşur. `edition` (Rust sürümü) anahtarı konusunu [Ek E](appendix-05-editions.html) bölümünde işleyeceğiz.

Son satırda projenizin bağımlılıklarını listelemeye yarayan [dependencies] bölümü yer alır. Rust'ta kodların paketler halinde tutulduğu yapılara `crate` yani sandık adı verilir. Bu proje için harici bir sandığa ihtiyaç duymayacak fakat 2. Bölümde gerçekleştireceğimiz ilk projede bağımlılıklar bölümünü kullanacağız.

Şimdi *src/main.rs* dosyasını açalım ve inceleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
fn main() {
     // "Merhaba, Cargo" olarak değiştirebilirsiniz.
    println!("Hello, world!"); 
}
```

Cargo sizin için tıpkı Örnek 1-1'de olduğu gibi ekranınıza "Merhaba, dünya!" metnini bastıran bir program oluşturdu. Önceki projemiz ile Cargo tarafından üretilen bu proje arasındaki farklar ise, Cargonun projeyi *src* adlı dizine yerleştirmesi ve üst dizinde ise bir *Cargo.toml* dosyası yaratması olarak özetlenebilir.

Cargo kaynak dosyalarının *src* dizininde bulundurulmasını bekler. Projenin ana dizin içeriği, sadece README dosyaları, lisans bilgileri, yapılandırma bilgileri ve kodunuzu ilgilendiren diğer şeyler içindir. Dolayısıyla Cargo, her şeyi ait olduğu dizine yerleştirerek düzenli projeler oluşturmanızı sağlar.


"Merhaba, dünya!" örneğinde yaptığımız gibi Cargo kullanılmadan başlatılan bir projeyi, tıpkı Cargo ile oluşturulmuş gibi düzenleyebilirsiniz. Bunun için proje kaynak kodunu *src* dizinine taşıyarak, projenin ana dizinde *Cargo.toml* dosyası oluşturmanız yeterlidir.

### Bir Cargo Projesini Derleyip Çalıştırmak

Şimdi "Merhaba, dünya!" programını Cargo kullanarak derleyip çalıştırdığımızda oluşan farklılıkları gözlemleyelim. Terminalinizde *merhaba_cargo* dizinine gelerek aşağıdaki komut yardımıyla projenizi oluşturun: 

```console
$ cargo build
    Compiling merhaba_cargo v0.1.0 (/home/rusdili/projeler/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 1.82s
```

Bu komut, *target/debug/merhaba_cargo* (veya Windows ortamında *target\debug\merhaba_cargo.exe*) 
konumunda çalıştırılabilir bir dosya oluşturur. Bu dosyayı şu komutla çalıştırabilirsiniz:

```console
$ ./target/debug/merhaba_cargo # veya Windows ortamında .\target\debug\merhaba_cargo.exe
Merhaba, Cargo!
```

Her şey yolunda giderse terminalinizde "Merhaba, Cargo!" yazısı görünecektir. Cargo uygulamasının ilk kez çalıştırılması projenizin ana dizininde *Cargo.lock* adında yeni bir dosya oluşturulmasına neden olur. Bu dosya projenizdeki bağımlılıkların tam sürümlerini takip eder. Halihazırda bu proje harici bir kasaya bağımlı olmadığından bu dosya epey boş görünecektir. Bu dosya içeriği Cargo tarafından otomatik olarak yönetildiğinden burada değişiklik yapmanız gerekmez.


Sadece *cargo build* komutu ile derlediğimiz ve *./target/debug/merhaba_cargo* komutu ile çalıştırdığımız bu projeyi, aynı anda derleyip çalıştırabilmek için `cargo run` komutunu kullanabiliriz. 

```console
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/merhaba_cargo`
Merhaba, Cargo!
```

Ancak bu defa Cargo'nun, `merhaba_cargo` programını derlediğini bildiren çıktının gösterilmediğine dikkat edin. Bu durum Cargo'nun kaynak kodun değiştilmediğini bilmesinden kaynaklanır. Kaynak kodunda değişiklik yaptığınmız programı Cargo ile yeniden derleyip çalıştıracak olursanız aşağıdakine benzer bir çıktı görürsünüz:

```console
$ cargo run
   Compiling merhaba_cargo v0.1.0 (/home/rusdili/projeler/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/merhaba_cargo`
Merhaba, Kargo!
```

Yine Cargo tarafından sağlanan ve kodunuzun çalıştırılabilir olup olmadığını denetleyen, fakat çalıştırılabilir dosyasını oluşturmayan `cargo check` adında bir komut daha vardır:

```console
$ cargo check
    Checking merhaba_cargo v0.1.0 (/home/rusdili/projeler/merhaba_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
```

Bazı durumlarda çalıştırılabilir dosya oluşturmanız gerekmez. `cargo check` komutu yürütülebilir dosya oluşturma adımını atladığından çoğu zaman `cargo build` işleminden daha hızlı olacaktır. Geliştirme aşamasında kodunuzun çalışıp çalışmadığını düzenli olarak kontrol ediyorsanız `cargo check` komutu üretim sürecinizi hızlandıracaktır. Pek çok Rustacean geliştirme aşamasında, programlarının derlendiğinden emin olabilmek için düzenli olarak `cargo check` kontrolü yapar. Ancak çalıştırılabilir dosyayı kullanmaya hazır olduklarında `cargo build` komutunu çalıştırırlar. 

Cargo hakkında şimdiye kadar öğrendiklerimizi özetleyecek olursak:

* Bir projeyi derlemek için `cargo build` komutunu kullanabiliriz.
* Bir projeyi tek bir adımla derleyip çalıştırabilmek için `cargo run` komutunu kullanabiliriz.
* Bir projenin hatalarını ikili kod üretmeden derleyerek kontrol edebilmek için  `cargo check` komutundan yararlanabiliriz.
* Cargo, derleme sonucunda oluşturulan çalıştırılabilir ikili dosyayı, kaynak koduyla aynı dizine koymak yerine *target/debug* dizinine kaydeder.

Cargo kullanmanın ek bir avantajı da, hangi işletim sisteminde çalışırsanız çalışın, kullanacağınız komutların değişmiyor olmasıdır. O nedenle bu noktadan itibaren Linux, macOS veya Windows işletim sistemlerinin her biri için ayrı talimatlar vermeyeceğiz.  

### Sürüm Amaçlı Derleme

Yayına hazır olan projenizi en iyileştirme olanakları ile derleyebilmek için `cargo build --release` komutunu kullanabilirsiniz. Bu komut çalıştırılabilir dosyanızı *target/debug* dizini yerine *target/release* dizinine çıkaracaktır. Fakat en iyileştirmeler, Rust kodlarının daha hızlı çalışmalarını sağlamakla birlikte, programın derlenmesi için gereken süreyi de uzatır. O nedenle en iyileştirme olanaklarında: İlki, hızlı ve sık derleme işlemleri için kullanılan geliştirme profili, diğeriyse tekrar tekrar derlenmeyecek ve çalışır halini olabilecek en kısa sürede kullanıcıya teslim edebileceğiniz nihai programı oluşturan olmak üzere iki farklı profil sunulur.

Eğer kodunuzun çalışma süresini ölçmek istiyorsanız, *target/release* dizininde bulunan çalıştırılabilir dosyayı `cargo build --release` komutu ile test ettiğinizden emin olun.

### Konvansiyonel Cargo

Cargo basit projelerde rustc kullanımından olduğundan fazla yarar sağlamıyor olsa da geliştirme süreci karmaşıklaştıkça değerini kanıtlayacak bir araç setidir. O nedenle birden çok sandıktan oluşan karmaşık projelerde koordinasyonu Cargo'ya devretmek oldukça faydalıdır.

Her ne kadar `merhaba_cargo` projesi basit bir projeymiş gibi görünüyor olsa da Rust kariyeriniz boyunca karşılaşacağınız gerçek araçların pek çoğunu kullanıyor. Var olan herhangi bir Rust projesinde çalışırken Git aracılığıyla kodu kontrol etmek, proje dizininine geçip kodu derlemek için aşağıdaki komutları kullanabilirsiniz. 

```console
$ git clone herhangibirurl.com/herhangibirproje
$ cd herhangibirproje
$ cargo build
```

Cargo hakkında daha fazla bilgi edinmek istiyorsanız [Cargo belgelerini](https://doc.rust-lang.org/cargo/) inceleyiniz. 

## Özet

Güzel başlayan Rust yolculuğunuzda aşağıdakileri öğrendiniz:

* Rust'ı, `rustup` kullanarak en son kararlı sürümüyle yüklemek.
* Rust'ı daha yeni bir sürümüne yükseltmek.
* Yerel olarak yüklenen belgelere erişmek.
* Bir "Merhaba, dünya!" programını yazarak bunu `rustc` kullanarak doğrudan çalıştırmak.
* Yeni bir projeyi Cargo komut ve kurallarını kullanarak derleyip çalıştırmak.

Şimdi Rust kodu okuyup yazma becerilerimizi geliştirebilmek için daha sağlam bir program yazmamız gerekiyor. Bu nedenle 2. Bölümde bir tahmin oyunu programı yazacak ve inceleyeceğiz. Eğer öğrenme sürecinize "Ortak Programlama Kavramları"nın nasıl çalıştığını öğrenerek devam etmek istiyorsanız 3. Bölüme ilerleyebilir, ardından 2. Bölüme geri dönebilirsiniz.    