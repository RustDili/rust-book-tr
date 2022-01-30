## Merhaba, Dünya

Artık Rust'ı yüklediğinize göre ilk Rust programımızı yazabiliriz. Yeni bir programlama dilini öğrenme aşamasında `Merhaba, dünya!` çıktısını ekrana yazdırmak neredeyse gelenek haline geldiğinden biz de burada bu geleneğe uyacağız.

> Not: Bu kitap komut satırı hakkında temel düzeyde bilgi sahibi olduğunuzu varsayar. Bununla birlikte Rust,
> kodlarınızı nasıl düzenleyeceğinize, hangi araçları kullanacağınıza veya onları nereye kaydedeceğinize 
> karışmadığından, dilerseniz komut satırı yerine aşina olduğunuz veya tercih ettiğiniz entegre 
> geliştirme ortamınızı (IDE) kullanabilirsiniz. 
> Son zamanlarda Rust ekibi farklı IDE'ler ile entegrasyonu iyileştirmeye odaklandığından artık birçok IDE belli
> düzeylerde dil desteği sağlıyor. 
> Tercih ettiğiniz IDE'nin yeterli dil desteği sağlayıp sağlamadığını IDE belgelerinden kontrol edebilirsiniz.

### Bir Proje Dizini Oluşturmak

Öncelikle işe Rust kodlarımızı saklayacağımız bir proje dizini oluşturarak başlayalım. Rust için kodunuzu nerede sakladığınız önemli olmamakla beraber, bu kitapta yer alan alıştırma ve projeler için ana dizininizde (linux için Home)yeni bir *projeler* dizini oluşturup tüm çalışmalarınızı orada depolamanızı öneririz.

Ana dizinde "Merhaba, dünya" projesinin saklanacağı *projeler* dizinin oluşturabilmek için bir terminal penceresi açarak sırasıyla aşağıdaki komutları uygulayalım.

Linux, macOS ve Windows PowerShell için aşağıdaki komutları girin:

```console
$ mkdir ~/projeler
$ cd ~/projeler
$ mkdir merhaba_dunya
$ cd merhaba_dunya
```

Windows CMD içinse şu komutları girin:

```console
> mkdir "%USERPROFILE%\projeler"
> cd /d "%USERPROFILE%\projeler"
> mkdir merhaba_dunya
> cd merhaba_dunya
```

### Bir Rust Programı Yazmak ve Çalıştırmak

Şimdi yeni bir kaynak dosyası oluşturun ve bunu *main.rs* olarak isimlendirin. Rust dosyaları daima `.rs` uzantısıyla sonlanır. Dosyalarınızı birden fazla kelime ile adlandırıyorsanız, bu adları `alt_çizgi` kullanarak birbirinden ayırın. Örneğin *merhabadunya.rs* yerine *merhaba_dunya.rs*'yi teercih edin. 

Az önce oluşturduğunuz *main.rs* dosyasını açarak Örnek 1-1'de yer alan kod satırlarını dosyanıza ekleyin:

<span class="filename">Dosya adı: main.rs</span>

```rust
fn main() {
	println!("Merhaba, dünya!");
}
```
<span class="caption"> Örnek 1-1: Ekrana "Merhaba, dünya!" yazdıran bir program.</span>

Dosyanızı kaydedip terminal penceresine geri dönün. Programımızı Linux veya macOS üzerinde derleyip çalıştırabilmek için aşağıdaki komutları uygulayalım:

```console
$ rustc main.rs
$ ./main
Merhaba, dünya!
```

Windows kullanıyorsanız `./main` yerine `.\main.exe` komutunu kullanmanız gerekir:

```powershell
> rustc main.rs
> .\main.exe
Merhaba, dünya!
```

Kullandığınız işletim sisteminden bağımsız olarak, terminalinizde `Merhaba, dünya!` çıktısını görüyor olmalısınız. Bu çıktıyı görmüyorsanız, yardım için kurulum bölümündeki ["Sorun Giderme"](ch01-01-installation.html#troubleshooting) başlığına göz atın.

Eğer `Merhaba, dünya!` satırını görüyorsanız **tebrikler**, bu sizin bir Rust programı yazdığınıza gösterir! Bir programcısı olarak aramıza hoş geldiniz.

### Bir Rust Programının Anatomisi

Ekranımıza `Merhaba, dünya!` yazısını bastıran programda neler olup bittiğine daha yakından bakalım. Bulmacanın ilk parçası aşağıdadır:

```rust
fn main() {
#	println!("Merhaba, dünya!");	

}
```

Rust'ta bu satırlar bir işlevi tanımlar. Çalıştırılabilir tüm rust programlarında bulunan `main` işlevi, programın işletilen ilk kodu olması bakımından özel bir konumdadır. İlk satır parametre almayan ve hiçbir şey döndürmeyen işlev adını `main` olarak bildirir. Eğer işleve parametre iletecek olsaydık, bu parametreleri `()` parantezin içine koymamız gerekirdi.

Ayrıca işlev gövdesinin süslü parantezlerle `{}` sarıldığıba dikkat edin. Rust'ta işlev gövdeleri bu süslü parantezler içine alınmak zorundadır. Girişi gösteren ilk süslü parantezi, işlev bildirimiyle aynı satıra yerleştirip bildirim ile aralarında bir boşluk bırakmak Rust'ın standart yazım biçimidir. 

Rust projelerinde standart yazım biçimine bağlı kalmak ve kodlarınızı belirli bir şekilde biçimlendirmek için `rustfmt` adındaki otomatik biçimlendirme aracını kullanabilirsiniz. Bu araç Rust ekibi tarafından tıpkı `rustc` gibi standart Rust dağıtımına dahil edildiğinden, halihazırda bilgisayarınızda kurulu olmalıdır. Daha fazla ayrıntı için çevrimiçi belgelere başvurabilirsiniz. 

Gövdesi süslü parantezler ile sarmalanmış olan `main` işlevinin içinde aşağıdaki kod satırı bulunur.

```rust
#fn main() {
	println!("Merhaba, dünya!");
#}
```

Bu küçük programdaki tüm işi üstlenerek metni ekrana yazdıran bu satırda dikkat edilmesi gereken dört önemli ayrıntı vardır.

İlki: Rust stili girintilerde bir sekme *(tab)* yerine dört boşluk *(space)* kullanılır.

İkincisi: `println!` terimi bir Rust makrosu çağırır. Eğer burada bir işlev çağrısı yapılıyor olsaydı, `println!` yerine (`!` olmadan) `println`  yazılmış olacaktı. Rust makrolarını 19. bölümde ayrıntılarıyla inceleyeceğiz. Ancak şimdilik `!` işaretininin normal bir işlev çağrısı değil, işlevler ile aynı kurallara uymayan bir makro çağrısı anlamına geldiğini bilmeniz yeterlidir.

Üçüncüsü: `"Merhaba, dünya!"` olarak gördüğünüz dizgi, `println!` makrosuna argüman olarak geçirildiğinde ekrana yazdırılır.

Ve sonuncusu: Satırın noktalı virgül (`;`) ile bitiyor olması, bu ifadenin bittiğini ve bir sonrakinin başlamaya hazır olduğunu bildirir. Rust kodlarındaki pek çok satır noktalı virgül ile biter.


### Derlemek ve Çalıştırmak Ayrı Birer Adımdır

Az önce oluşturduğunuz yeni programınızın çalışma sürecindeki adımlarını incelelim 

Bir Rust programı çalıştırılmadan önce Rust derleyicisi kullanılarak ve `rustc` komutuna aşağıdaki gibi kaynak dosyası adı verilerek derlenmelidir:

```console
$ rustc main.rs
```

C veya C++ dillerine aşinaysanız, bu işlemin `gcc` veya `clang` ile benzeştiğini fark edeceksiniz. Başarıyla gerçekleşen bir derlemenin ardından Rust çalıştırılabilir ikili (binary) bir dosya üretecektir.

Bu çalıştırılabilir dosyaya, Linux, macOS veya Windows PowerShell sistemlerinde, dizin içindeyken terminalinize `ls` komutu girerek ulaşabilirsiniz. Linux ve macOS sistemlerinde aynı dizinde iki adet dosya görünürken, Windows  PowerShell'de CMD kullanıldığında üç dosya görüntülenecektir.

```text
$ ls
main main.rs
```

Eğer Windows üzerinde CMD kullanıyorsanız aşağıdaki komutu girmeniz gereklidir:

```cmd
> dir /B %= Buradaki /B seçeneği yalnızca dosya isimlerinin görüntülenmesini sağlar =%
main.exe
main.pdb
main.rs
```

Her iki durumda da *.rs* uzantılı bir kaynak kodu dosyası, (windows'ta *main.exe* olarak ancak diğer platformlarda sadece *main* olarak görünen) çalıştırılabilir ikili dosya, Windows için ek olarak hata ayıklama bilgilerini içeren *.pdb* uzantılı birer dosya gösterilecektir.

Bu dizinde çalıştırılabilir halde bulunan *main* ya da *main.exe* dosyasını aşağıdaki gibi kullanarak işletebilirsiniz:

```console
$ ./main # ya da windows için .\main.exe
```

Eğer `main.rs` dosyanız "Merhaba, dünya!" programınızı içeriyorsa terminalinize `"Merhaba, dünya!"` metni yazdıracaktır.

Programlama tecrübeniz Ruby, Python veya JavaScript gibi dinamik dillerden oluşuyorsa bu programın ayrı adımlar halinde derleyip çalıştırılmasına alışkın olmayabilirsiniz. Ancak Rust *ahead-of-time compiled* (öncesinde derlenmiş)  bir dildir. Bu derlenmiş bir Rust programının yürütülebilir dosyasının dağıtılabileceği ve dağıtılan bu dosyanın Rust kurulumuna ihtiyaç duymadan çalıştırılabileceği anlamına gelir. Ancak bir `rb, .py`, veya `.js` dosyası dağıttığınızda bu dosyanın kullanılacağı ortamda bir Ruby, Python veya JavaScript uygulamasının yüklü olması gerekir. Bununla birlikte bu dillerden biriyle yazılmış olan bir programı çalıştırabilmek için yalnızca bir komutun kullanılması yeterlidir. Dil tasarımında her şey bir değiş tokuştur.

Her ne kadar basit programların `rustc` ile derlenmesi yeterliymiş gibi görünse de projeniz büyüdükçe seçeneklerin tümünü yönetmek ve kodun dağıtılmasının kolaylaştırmak isteyeceksiniz. Sonraki bölümde sizi gerçek dünyada daha sık kullanılan ve daha karmaşık Rust programları yazmanıza yardım edecek olacak Cargo aracıyla tanıştıracağız.