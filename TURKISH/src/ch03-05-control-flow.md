## Kontrol Akışı

Bir kodun doğruluğuna bağlı olarak kodun başka bir bölümünün çalıştırılması veya bazı kodların bir koşul doğru olduğu sürece çalıştırılması çoğu programlama dilinin temel yapı taşlarıdır. Rust'ta kod yürütme akışını kontrol edebilmenizi sağlayan en yaygın kontrol yapıları `if` ifadeleri ve `loop` döngüleridir.

### `if` İfadeleri

`If` ifadesi kodunuzu koşullara göre bölerek yürütmenize olanak sağlar. Bir koşul belirleyip ardından "Bu şart sağlanırsa şu kod bloğunu çalıştırın, koşul sağlanmıyorsa çalıştırmayın." demeye benzer.

`If` ifadesini daha iyi kavrayabilmek için *projeler* dizininde *dallar* adında yeni bir proje oluşturup *src/main* dosyasına aşağıdaki kodları ekleyin: 

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Tüm `if` ifadeleri, `if` anahtar kelimesiyle başlar ve bunu bir koşul takip eder. Örneğimizdeki koşul `sayı` değişkeninin 5'ten küçük olup olmadığını kontrol eder. Koşulun `true` (doğru) olması durumunda yürütülecek kod bloğu koşulun hemen ardından eklenen süslü parantezler içine yerleştirilir. `If` ifadesinin koşulunu denetleyen kod blokları, kitabın 2. Bölümündeki [Tahmin Sayısının Gizli Sayı ile Karşılaştırılması][comparing-the-guess-to-the-secret-number]<!-- ignore -->  konusunda yer alan eşleme ifadesinde olduğu gibi bazen kol bazen dal olarak adlandırılabilir.

Dilerseniz örnekte yaptığımız gibi, koşulun `false` (yanlış) olması halinde alternatif bir kod bloğu olarak işletilecek bir `else` ifadesini de kodunuza ekleyebilirsiniz. Eğer koşul yanlış ve bir `else` ifadesi bildirilmemişse program `if` bloğunu geçerek sonraki kod parçasına atlayacaktır.

Kodu çalıştırdığınızda aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/output.txt}}
```

Ne olacağını görmek için `sayı` değerini, koşulu `false` yapacak bir değerle değiştirelim:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/src/main.rs:here}}
```

Ve programı yeniden çalıştırıp çıktıyı inceleyin:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-27-if-false/output.txt}}
```

Bu koddaki koşulun bir `bool` olması gerektiğini belirtmekte yarar var. Koşulun `bool` olmaması halinde hata ile karşılaşırız. Örnekteki kodu çalıştırmayı deneyin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/src/main.rs}}
```

Artık `if` koşulu 3 değerine ayarlanmış olduğundan Rust bir hata döndürecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-28-if-condition-must-be-bool/output.txt}}
```

Bu hata bize, Rust'ın `bool` türünde bir değer beklediğini ancak tam sayı türünde değer aldığını gösterir. Ruby veya Javascript gibi dillerin tersine Rust, boolean olmayan türleri otomatik olarak boolean türüne dönüştürmeye kalkışmaz. Açık olmanız ve `if` koşulunun daima boolean olmasını sağlamanız gerekir. Örneğin sayı `0` olmadıkça `if` kod bloğunun yürütülmesini istiyorsak, `if` ifadesini aşağıdaki gibi değiştirebiliriz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-29-if-not-equal-0/src/main.rs}}
```

Bu kod çalıştırıldığında ekrana `Bu sayı sıfır değil!` mesajını yazdıracaktır.

#### `else if` ile Koşulları İşlemek

Bir `else if` ifadesinde `if` ve `else` kelimelerini birleştirerek çok sayıda koşulu denetleyebilirsiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Bu programın gidebileceği dört olası yol vardır. Progamı çalıştırdığınızda aşağıdaki çıktıyı görmelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/output.txt}}
```

Program yürütüldüğünde `if` ifadelerinin her birini sırayla kontrol edecek ve bulduğu ilk doğru koşulu işletecektir. 6 sayısının 2'ye kalansız bölünüyor olmasına rağmen, çıktıda `Sayı 2' ye kalansız bölünebilir.` mesajını veya `else` bloğunda yer alan `Sayı 4, 3 veya 2'ye kalansız bölünemez!` mesajını görmediğimize dikkat edin. Bunun nedeni Rust'ın kontrol sırasındaki ilk doğru koşulu bularak onu işletmesi ve diğer koşulların doğu olup olmamasıyla ilgilenmemesidir. 

Çok sayıda `else if` ifadesi kullanmak kodunuzu karıştırabilir. Gereğinden fazla `else if` ifadesi kullandığınızı düşünüyorsanız kodunuzu yeniden düzenlemelisiniz. Kitabın 6. bölümünde böyle durumlarda kullanabileceğiniz güçlü bir dallanma yapısına sahip `match`  (eşleme) adlı bir yapı anlatılır. 

#### Bir `let` Deyiminde `if` Kullanmak 

Örnek 3-2'de olduğu gibi `if`'in bir ifade olması, sonucunun herhangi bir değişkene atanmak üzere `let` deyiminin sağ tarafında kullanabilmesni sağlar.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

<span class="caption"> Örnek 3-2: Bir `if` ifadesi sonucunu değişkene atamak.</span>

`Sayı` değişkeni `if` ifadesinin sonucuna göre oluşan bir değere bağlanacaktır. Bu kodu çalıştırdığınızda aşağıdaki çıktıyı elde edeceksiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-02/output.txt}}
```

Kod bloklarının, içlerinde bulunan son ifadeyi değerlendirdiğini ve sayıların da birer ifade olduğunu unutmayın. Bizim durumumuzda tüm `if` ifadesinin değeri yürütülecek olan kod bloğunun değerine bağlıdır. Bu da, `if` ifadesindeki sonuç üretme potansiyeline sahip her dalın aynı türden olması gerektiği anlamına gelmektedir. Örnek 3-2'de bulunan `if` ve `else` dallarının her biri `i32` türünde birer tam sayıdır. Aşağıdaki örnekten de anlaşılacağı gibi, türlerin uyumsuz olması hata alınmasına neden olur:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/src/main.rs}}
```

Kodu derlemeye çalıştığımızda `if` ve `else` kollarının uyumsuz türlerden oluştuğu ve bu hatanın hangi satırda bulunduğunu gösteren bir hata raporuyla karşılaşırız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-31-arms-must-return-same-type/output.txt}}
```

`If` bloğundaki ifade tam sayı olarak değerlendirilirken `else` bloğundaki ifadeyse dizgi olarak olarak değerlendirilecektir. Değişkenlerin aynı türden olması ve `sayı` değişkeni türünün Rust tarafından derleme zamanında kesinlikle biliniyor olması gerektiğinden bu kod işe yaramaz. `Sayı` türünün derleme zamanında biliniyor olması, bu değişkenin kullanıldığı her yerde, derleyici tarafından türünün doğru ve geçerli olduğunun garantilenmesini sağlar. Eğer `sayı` değişkeninin türü sadece çalışma zamanında belirlenmiş olsaydı; herhangi bir değişken için çok sayıda varsayımsal türün takip edilmesi gerekecek, buna bağlı olarak derleyici karmaşıklaşacak ve kod hakkında daha az garanti verebileceğinden Rust bunu yapamamış olacaktı.

### Döngüler ile Tekrarlama

Bazen bir kod bloğunu defalarca çalıştırmak gerekir. Rust bu amaçla döngü gövdesi içinde kalan kodun tamamını çalıştırıp hemen ardından yeniden baştan başlatan çeşitli *döngüler* sağlar. Döngülerle çalışabilmek için *projeler* dizininde *donguler* adında yeni bir proje başlatalım.

Rust'ta `loop`, `while` ve `for` olmak üzere üç çeşit döngü vardır. Bunların her birini birlikte deneyelim:

#### `loop` ile Kod Tekrarı

Bir anahtar sözcük olan `loop` Rust'a, ait olduğu kod bloğunu sonsuza dek ya da siz onu açıkça durdurana kadar tekrar tekrar çalıştırmasını söyler. Şimdi *donguler* dizinindeki *src/main.rs* dosyasını örnektekine benzer şekilde değiştirin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Programı çalıştırdığınızda terminalinizi elle kapatana kadar `Tekrar!` mesajının yazdırıldığını göreceksiniz. Pekçok terminal sonsuz döngüye kapılan programların sonlandırılmasını sağlayan <span class="keystroke">ctrl+c</span> klavye kısa yolunu destekler. Demeyelim:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-32-loop
cargo run
CTRL-C
-->


```console
$ cargo run
   Compiling donguler v0.1.0 (/home/rusdili/projeler/donguler)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/donguler`
Tekrar!
Tekrar!
Tekrar!
Tekrar!
^CTekrar!
```

`^C` işareti <span class="keystroke">ctrl-c</span> tuşuna bastığınız yeri gösterir. `^C`'den sonra `Tekrar!` yazısını görmeniz, kodun kesme sinyalini aldığında döngünün neresinde bulunduğuna bağlı olduğundan bu mesajı göremeyebilirsiniz.

Neyse ki Rust, bu tür döngülerden kod kullanarak çıkmanın bir yolunu sağlar. Programa işletilen döngünün durdurulacağı yeri, o noktaya bir `break` anahtar sözcüğü yerleştirerek bildirebilirsiniz. Bu yöntemi 2. Bölümdeki [“Doğru Tahmin Sonrası Oyundan Çıkmak”][quitting-after-a-correct-guess]<!-- ignore --> bölümünden hatırlayor olmanız gerek.


Yine hatırlayacağınız gibi tahmin oyunu programında, döngünün o anki tekrarını durdurup bir sonraki tekrara atlayan `continue` anahtar kelimesini de kullanmıştık. 

İçiçe döngüler söz konusu olduğunda `break` ve `continue` anahtar kelimeleri en içteki döngüye uygulanır. Dilerseniz döngü üzerinde daha sonra `break` ya da `continue` ile kullanabileceğiniz bir *döngü etiketi* bildirebilirsiniz. Bu durumda `break` ve `continue` anahtar kelimeleri en içteki döngüye değil etiketlenen döngüye uygulanırlar. Aşağıda iki adet içiçe geçmiş döngü örneği yer almaktadır:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

`saydır` etiketine sahip olan dış döngü 0'dan 2'ye kadar sayar. Etiketsiz olan iç döngü ise 10'dan 9'a doğru geri sayım yapar. Etiketsiz olan ilk `break` yalnızca iç döngüden, `break 'saydır;` ifadesiyse dış döngüden çıkar. Bu kod aşağıdaki çıktıyı üretir:

```console
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/output.txt}}
```

#### Döngülerden Değer Döndürmek
 
Döngü kullanımlarından biri de, iş parçacıklarının işlerini bitirip bitirmediğinin kontrolü gibi başarısız olması muhtemel işlemleri yeniden denemektir. Hem ayrıca işlem sonucunu bu döngünün dışında kalan kod bölümüne de aktarmanız gerekebilir. Bunu yapabilmek için döngüyü döngüyü sonlandıracak olan `break` ifadesinden ardından döndürülmesini istediğiniz değeri eklemek yeterlidir. Bu değer örnekte gösterildiği gibi döngüden döndürülecektir:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Döngüden önce `sayaç` adında bir değişken tanımlıyarak `0` değeriyle başlatıyoruz. Hemen ardından döngüden dönecek olan değeri depolayacağımız `sonuç` değişkenini tanımlıyoruz. Döngü tekrarlandıkça sayaç değerine `1` ekleneceğinden sayacın `10`'a eşit olup olmadığını kontrol ediyor, değer `10` olduğunda `break` anahtar sözcüğüne ek olarak `sayaç * 2` değerini ekliyoruz. Döngü bitiminde değeri `sonuç` değişkenine atayacak olan ifadeyi noktalı virgül ile bitirdiğimize dikkat edin. Nihayetinde programı `sonuç` değişkenine atanan 20 değerini yazdıracak şekilde tamamlayıp bitiriyoruz.

#### `while` ile Koşullu Döngüler

Programların genellikle döngü içinde bulunan koşulları değerlendirmeleri gerekir. Koşul doğru olduğu sürece çalışan döngü, koşulun yanlış olması durumunda programın `break` çağrısı sonucunda durdurulur. Bu tür bir davranışı `if`, `else` ve `break` kombinasyonlarını kullanarak uygulamak mümkündür. Eğer isterseniz bunu bir programla hemen şimdi deneyebilirsiniz. Fakat bu model o kadar yaygın biçimde kullanılmaktadır ki, Rust bunun için `while` döngüsü adında yerleşik bir dil yapısı sunar. Örnek 3-3'te geriye doğru 3 tur dönen ve her dönüşünde döngünün bulunduğu turu yazdıran, son olarak bir mesaj yazdırarak döngüden çıkan program için `while` döngüsünden yararlanıyoruz. 

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

<span class="caption">Örnek 3-3: Koşul doğru olduğu sürece çalışan kod için `while` döngüsünü kullanmak</span>

Bu yapı, `loop`, `if`, `else` ve `break` kullanarak yazacağınız bir programda gerekli olacak çok sayıda içiçe yuvalanmayı ortadan kaldıracağı için oldukça nettir. Ve bu kod, koşul doğru olduğu sürece çalışacak aksi halde döngüden çıkacaktır.

#### Bir Koleksiyonu `for` Döngüsüyle Dolaşmak

Dizi gibi bir koleksiyonun öğeleri üzerinde yineleme yapmak için `while` yapısını kullanmak isteyebilirsiniz. Mesela Örnek 3-4'teki döngü `a` dizisindeki tüm öğeleri yazdırır:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-04/src/main.rs}}
```

<span class="caption">Örnek 3-4: Bir koleksiyonun öğelerini `while` döngüsü kullanarak dolaşmak</span>

Bu kod dizideki elemaları sayar. Bunu 0 dizininden başlayarak koleksiyonun sonuncu dizinine yani koşulumuz `dizin < 5`'in doğru olmadığı noktaya dek döngü şeklinde tekrarlayarak yapar. Bu kod çalıştırıldığında dizideki tüm öğeler yazdırırılır:

```console
{{#include ../listings/ch03-common-programming-concepts/listing-03-04/output.txt}}
```

Beklendiği gibi dizideki beş elemanın her biri terminalde görünür. Bir noktada `dizin` değeri `5`'e ulaşsa bile, diziden altıncı değeri alınmadan önce döngü yürütmeyi durdurur.

Ancak bu yaklaşım dizin değeri ya da test koşulunun yanlış olduğu hallerde hataya açık olup programın paniklemesine neden olur. Örneğin eğer a dizisini 4 elemandan oluşacak şekilde yeniden düzenler ve döngü koşulunu `dizin < 4` şeklinde güncellemeyi unutursanız kodunuz panikleyecektir. Ayrıca bu tasarım derleyicinin, döngü boyunca her tekrarda koşulun dizi sınırlarını aşıp aşmadığını kontrol edecek ek çalışma zamanı kodları eklemesini gerektireceğinden yavaş kalacaktır.

Alternatif olarak bir koleksiyondaki her öğeyi ayrı ayrı işlemek için daha kısa ve özlü olan `for` döngüsünü kullanabilirsiniz. Bir `for` döngüsü Örnek 3-5'teki koda benzer:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

<span class="caption">Örnek 3-5: Bir koleksiyonun öğelerini `for` döngüsü kullanarak dolaşmak</span>

Bu kodu çalıştırdığımızda Örnek 3-4'tekiyle aynı mesajları alırız. Daha da önemlisi artık kodun güvenliğini artırmış, dizi eleman sayısının ötesine geçmek ya da gereği kadar tur yapmamaktan kaynaklı bazı öğelerin işlenememesi gibi hata olasılıklarını ortadan kaldırmış olduk.

Hem ayrıca `for` döngüsü kullanımında dizi öğe sayısının değişmesi, Örnek 3-4'te olduğu gibi kodun yeniden güncellenmesini gerektirmez.

Kısa ve güvenle kullanılıyor olması `for` döngüsünün Rust'ta en yaygın kullanılan döngü yapısı olmasını sağlar. Geri sayım için `while` döngüsü kullanan Örnek 3-3'te olduğu gibi pekçok Rust geliştiricisi, belli sayıda tekrarlanacak kodlar için bile `for` döngüsünden yararlanır. Geliştiriciler bunu yaparken, belli bir başlangıç ve bitiş sayısı arasında kalan tüm sayıları sırayla üreten ve standart kitaplık tarafından sağlanan bir `Range` aralığı kullanırlar.

Aralığı tersine çevirebilmek içinse aşağıda gösterildiği gibi for döngüsü eşliğinde henüz görmediğimiz `rev` metodu kullanılır:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

Bu kod size daha hoş görünmüyor mu?

## Özet

Değişkenler, skaler ve bileşik veri türleri, işlevler, yorumlar, `if` ifadeleri ve döngüleri içeren oldukça büyük bir bölümü biigi sahibi olup bölümü tamamladınız. Burada tartışılan kavramları pekiştirmek amacıyla sonraki satırda önereceğimiz programları yazmayı deneyin.

* Isı değerlerini Fahrenheit ve Celsius dereceleri arasında dönüştürün.
* Fibonacci serisindeki n. eleman değerini hesaplayın.
* Bir noel şarkısı olan "The Twelve Days of Christmas"ın nakaratlarını kullanarak şarkının sözlerini yazdırın.

Devam etmeye hazır olduğunuzda diğer programlama dillerinde *olmayan* Rust'ın mülkiyet kavramından bahsedeceğiz. 

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]:ch02-00-guessing-game-tutorial.html#dogru-tahmin-sınrasi-oyundan-cikmak