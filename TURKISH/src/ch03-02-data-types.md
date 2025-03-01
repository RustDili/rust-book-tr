## Veri Türleri

Her değerin belirli bir veri türüne ait olması Rust'a ne tür verilerin bildirildiği ve bu verilerin nasıl işleneceğini söyler. Bu başlıkta skaler ve bileşik olmak üzere iki *veri türü* alt kümesine odaklanacağız.

Rust'ın *statik olarak yazılmış* bir dil olduğunu ve tüm değişken türlerinin derleme sırasında  biliniyor olması gerektiğini unutmayın. Derleyici genellikle değere ve onu nasıl kullandığımıza bağlı olarak kullanmak istediğimiz türü anlayabilir. Ancak çıkarsanabilecek farklı türlerin olması durumunda, kitabımızın 2. bölümünde yer alan ["Tahmin Sayısının Gizli Sayı ile Karşılaştırılması"][comparing-the-guess-to-the-secret-number]<!-- ignore -->  bölümünde String türünü sayısal bir türe dönüştürürken yaptığımız gibi tür ek açıklaması eklememiz gerekir:

```rust
let tahmin: u32 = "42".parse().expect("Lütfen bir sayı türü girin!");
```

Böyle bir ifadeye tür ek açıklaması eklenmezse Rust derleyicisi aşağıdaki gibi, kullanılmak istenen türün açıkça bildirilmesi gerektiğini söyleyen bir hata döndürecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/output-only-01-no-type-annotations/output.txt}}
```

Diğer veri türleri için farklı tür ek açıklamaları göreceksiniz.


### Skaler Veri Türleri

Bir *skaler* tür tek bir değeri temsil eder. Rust'ta dört ana skaler tür bulunur: Tamsayılar, kayan noktalı sayılar, boolean'lar ve karakterler. Diğer programlama dillerinden aşina olduğunuz bu türlerin Rust'ta nasıl çalıştığını inceleyelim.

#### Tamsayı Türleri

Tamsayılar kesirli bileşeni olmayan sayılardır. Hatırlarsanız kitabımızın 2. Bölümünde `u32` türünde bir tamsayı kullanmıştık. Bu tür bildirimi, ilişkilendirildiği değerin bellekte 32 bitlik bir alanı kaplayan işaretsiz bir tamsayı olması gerektiğini belirtir. İşaretli tamsayılar `i`, işaretsiz tamsayılar ise `u` ön ekini alırlar. Tablo 3-1, Rust'ın yerleşik olan tamsayı türlerini göstermektedir. Tıpkı `i16` örneğinde olduğu gibi. **İşaretli** ve **İşaretsiz** sütunlardaki her seçenek, bir tamsayı değerinin türünü bildirmek için kullanılabilir.

<span class="filename">Tablo 3-1: Rust'ın Tamsayı Türleri</span>

| Uzunluk  | İşaretli  | İşaretsiz |
|----------|-----------|-----------|
| 8-bit    |   `i8`    |   `u8`    |
| 16-bit   |   `i16`   |   `u16`   |
| 32-bit   |   `i32`   |   `u32`   |
| 64-bit   |   `i64`   |   `u64`   |
| 128-bit  |   `i128`  |   `u128`  |
| arch     |   `isize` |   `usize` |

Her varyant işaretli veya işaretsiz olabileceği gibi bit cinsinden belirli bir boyuta sahiptir. Varyantın işaretli olması sayının negatif değerler alabileceğini, işaretsiz olmasıysa sayının yalnızca pozitif olabileceği anlamına gelmektedir. Başka bir ifadeyle, sayının bir işaretli alması gerekip gerekmediğini *işaretli*, sayının sadece pozitif olacağını ve bir işaret ile gösterilmesi gerekmediğiniyse *işaretsiz* sayılar temsil eder. Bir sayıyı kağıda yazarken yaptığımız gibi, işaretin önemli olduğu hallerde sayıyı +, veya - olarak işaretlememize, pozitif olduğu hallerdeyse işaretsiz koymadan kullanmamıza benzer. İşaretli sayılar [ikinin tümleyeni](https://tr.wikipedia.org/wiki/%C4%B0kinin_t%C3%BCmleyeni)<!-- ignore --> gösterimi kullanılarak depolanır.

Her işaretli varyant -(2<sup>ⁿ⁻¹</sup>) ile 2<sup>ⁿ⁻¹</sup>-1 arasındaki sayıları depolayabilir. Formüldeki *n* ise varyantın kullandığı bit sayısını gösterir. Bu bir `i8` varyantının -(2<sup>⁷</sup>) ile 2<sup>⁷</sup>-1 arasındaki sayıları yani -128 ile 127 değerleri arasındaki sayıları depolayabileceği anlamına gelir. İşaretsiz varyantlar ise 0 ile 2<sup>ⁿ⁻¹</sup> arasındaki sayıları saklayabildiklerinden, bir `u8` varyantının 0 ile 2<sup>⁸</sup> - 1, yani 0 ile 255 arasındaki sayıları depolayabilirler.

Ek olarak boyutları ve kullanım türleri programın çalıştığı bilgisayar mimarisine bağlı olan `isize` ve `usize` türleri vardır. Bunlar 64 bit mimari kullanıyorsanız 64, 32 bit mimari kullanıyorsanız 32 bit olarak değerlendirilirler.

Tamsayı değişmezlerinizi tablo 3-2'de gösterilen biçimlerden herhangi biriyle yazabilirsiniz. Bayt değişmezi haricindeki tüm değişmez değerlerin, `57u8` gibi bir tür son ekine ve `1_000` örneğinde olduğu gibi görsel bir ayırıcı olarak `_` kullanmanıza izin verdiğini unutmayın.

<span class="caption">Tabl0 3-2: Rust'taki Tamsayı Değişmezleri</span>

| Sayısal Değişmez | Örnek         |
|------------------|---------------|
| Onluk            | `98_222`      |
| Onaltılık        | `0xff`        |
| Sekizlik         | `0o77`        |
| İkilik           | `0b1111_0000` |
| Bayt (sadece`u8`)| `b'A'`        |

Tam sayı türlerini seçerken kararsız kaldığınızı hissederseniz Rust'ın varsayılan türleri ile devam edebilirsiniz. Rust'ta tam sayılar için varsayılan tür `i32` 'dir. Bazı koleksiyonları indexlenmesi gerekiyorsa bunun için genellikle `isize` veya `usize` türü kullanılır.

> ##### Tamsayı Taşması
>
> 0 ile 255 arasında değerlere sahip olabilen `u8` türünde bir değişkeniniz olduğunu varsayalım. Değişkeni bu aralığın
> dışında, örneğin 256 gibi türün tutabileceği en yüksek değerden fazla bir değere ayarlamaya kalkışırsanız tamsayı taşmasına
> neden olursunuz. Bu gibi davranışlar karşısında Rust derleme seçeneklerinde yer alan `debug` *(hata ayıklama)* seçeneği, programı çalışma
> zamanında panikletmek üzere tamsayı taşması kontrolleriyle donatılmıştır. Rust'ta *panic yapmak* veya *programın paniklemetilmesi*, 
> oluşan bir hata sebebiyle programdan çıkılması anlamına gelmektedir.
>  
> Bu konuya kitabımızın 9. Bölümünde ["`panic!` ile Düzeltilemeyen Hatalar"][unrecoverable-errors-with-panic]<!-- ignore -->
> başlığında odaklanacağız.
>
> Tam sayı taşması kontrolleri `--release` *(yayın seçeneği)* ile yapılan derlemelerde gerçekleştirilmez. Ancak taşma 
> oluşması halinde taşan değerler Rust tarafından ikinin tümleyeni yöntemiyle sarmalanarak, türün sahip olduğu en küçük 
> değerden başlayıp ileriye doğru kaydırılır. Taşmanın `u8` türünde olduğunu varsaydığımızda bu kaydırmalar, 256 değeri 
> için 0'a, 257 değeri içinse 1'e evrilir ve rakam yükseldikçe bu böyle devam eder. Nihayetinde program paniklemeyecek ancak değişken
> büyük bir ihtimalle beklenmeyen bir değere sahip olacaktır. Tam sayı taşmaları için sarmalama yöntemine güvenmek hata olarak 
> kabul edilmektedir.
>
> Standart kitaplık tarafından sağlanan aşağıdaki yöntemleri temel türlerdeki taşma olasılıklarını yönetmek amacıyla kullanabilirsiniz. 
>
> - Tüm modları `wrapping_add` gibi [`wrapping_*`][wrapping]<!-- ignore --> metodlarıyla sarmalayın.
> - Taşmanın gerçekleşebileceği durumları `checked_*` metodlarıyla denetleyip `None` değeri döndürecek şekilde yönetin.
> - `overflowing_*` gibi taşma oluşup oluşmadığını, bir *boolean* değer döndürerek gösteren metodlarından yararlanın.
> - `saturating_*` gibi en yüksek ve en düşük değerleri için doyurucu aritmetik işlemlerinden yararlanan metodlardan faydalanın.

#### Kayan Noktalı Türler

Rust ayrıca ondalık basamaklı sayılar olarak bildiğimiz kayan noktalı sayılar için iki temel tür sağlar. Bunlar sırasıyla 32 bit boyutunda olan `f32` ve 64 bit boyutunda olan `f64` türleridir. Her iki tür de modern CPU'larda eşit hızlarda çalıştığından, kayan noktalı sayılar için Rust'ın varsayılanı daha yüksek duyarlıktaki `f64` türüdür. Her iki kayan noktalı sayı türü de işaretlidir.

Aşağıdaki örnek kayan noktalı sayıların işleyişini göstermektedir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-06-floating-point/src/main.rs}}
```

Kayan noktalı sayılar IEEE-754 standardına göre temsil edilir. Buna göre `f32` tek, `f64` ise çift hassasiyetli türlerdir.

#### Sayısal İşlemler

Rust tüm sayı türleri için ihtiyaç duyabileceğiniz, toplama, çıkarma, çarpma, bölme, kalan gibi temel matematik işlemlerini destekler. Tam sayılar bölündüğünde bir altındaki en yakın sayıya yuvarlanır. Aşağıdaki örnek türlerin `let` ifadeleriyle nasıl kullanılabileceğini göstermektedir: 


<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-07-numeric-operations/src/main.rs}}
```

Her ifade metematiksel işleçler kullanarak ilgili değişkene atanacak olan benzersiz bir değeri hesaplar. Rust'ta yer alan  matemetiksel işleçler bu kitabın [EK B][appendix_b]<!-- ignore --> bölümünde listelenmektedir.

#### Boolean Türü

Çoğu programlama dilinde olduğu gibi Rust'taki boolean türü de `true` ve `false` olmak üzere bir baytlık iki olası değerden birine sahiptir. Bu tür Rust'ta `bool` olarak belirtilir. Örneğin:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-08-boolean/src/main.rs}}
```

Boolean değerleri genellikle `if` gibi koşullu ifadelerle kullanılır. Bu ifadenin çalışma şeklini ["Kontrol Akışı"][control-flow]<!-- ignore --> bölümünde ele alacağız.

#### Karakter Türü

Rust'ın karakter türü dilin en temel alfabetik türüdür ve kullanılışı aşağıdaki gibi örneklenebilir.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-09-char/src/main.rs}}
```

Çift Tırnak kullanan dizgi değişmezlerinin tersine `char` değişmezleri tek tırnakla bildirilir. Rust'ın `char` türü dört baytlık bir Unicode skaler değeriyle temsil edildiğinden, ASCII karakter tablosunda bulunandan daha fazla karakteri barındırır. Aksanlı harfler, Türk, Çin, Japon, Kore dilindeki karakterler, emoji ve sıfır genişlik boşukların tamamı Rust'ta geçerli `char` değerlerdir. Unicode skaler değerleri `U+0000` ile `U+D7FF` ve `U+E000` ile `U+10FFFF` arasında değişir. Ancak "karakter" kavramı Unicode için gerçek bir kavram olmadığından, karakterin anlamına dair insan sezgisi ile Rust'taki karakterin anlamı tam olarak uyuşamayabilir. Bu konuyu 8. Bölümde ["UTF-8 Kodlu Metni Dizgilerde Saklamak"][strings]<!-- ignore --> bölümünde ayrıntılarıyla inceleyeceğiz.

### Bileşik Veri Türleri

Bunlar çok sayıda değeri tek bir tür olarak gruplayabilen türlerdir. Rust'ta diziler ve çokuzlular olmak üzere iki temel *bileşik tür* bulunur.

#### Demet Türü

Demetler, çeşitli türlerden oluşan bir dizi değeri, tek bir bileşik tür halinde bir araya getirmenin genel yoludur. Bu türün boyutları sabittir ve bildirildikten sonra değiştirilemez.

Virgülle ayrılmış değerler listesinin parantez içine yazılmasıyla oluşturulur. İçeriğin her konumu bir türü temsil ettiğinden, bir araya getirilen değerler farklı türlerden seçilebilir. Örnekteki tür açıklamaları isteğe bağlı olarak eklenmiştir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-10-tuples/src/main.rs}}
```

Demetler tek bir bileşik tür olarak kabul edildiğinden tüm değerler `demet` değişkenine bağlanır. Demeti oluşturan her öğeye ayrı ayrı erişebilmek için onu çözmekte kullanılan bir çeşit örüntü eşleme modeli olan *destructuring* yönteminden yararlanırız.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-11-destructuring-tuples/src/main.rs}}
```

Bu programda önce bir demet oluşturarak onu `demet` değişkenine bağlarız. Ardından bu demeti, `x`, `y` ve `z` adlarında üç ayrı değişkene dönüştürebilmek için `let` ifadesinden yararlanırız. Bu işlem, demetin öğelerine ayrılmasına ve her öğesinin ayrı bir değişkene atanmasına sebep olduğundan *çözme, yıkma* anlamına gelen *destructuring* olarak adlandırılır. Nihayetinde son satırda, elde ettiğimiz değişkenlerden `y`'nin tuttuğu değer olan `6.4`'ü yazdırarak programı tamamlarız.

Bu yöntemine ek olarak demet elemanlarına isminden hemen sonra bir  (`.`) nokta ve öğe dizin numarası yazarak yani nokta gösterimini kullanarak doğrudan da erişebiliriz. Aşağıdaki örnek bununla ilgilidir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-12-tuple-indexing/src/main.rs}}
```

Bu programda `x` adında bir demet oluşturup bu demetin her elemanını için bir dizin numarası kullanarak onlardan yeni değişkenler üretiriz. Bir demetin dizin numarası çoğu programlama dilinde olduğu gibi 0'dan başlar.

Hiç bir değere sahip olmayan `()` boş bir demet yalnızca bir değere sahip özel bir türdür ve `()` şeklinde yazılabilir. Bu türe *birim türü* değerine ise *birim değeri* adı verilir. Hiç bir değer döndürmeyen ifadeler örtük olarak birim değer döndür.

#### Dizi Türü

Çok sayıda değerden oluşan bir koleksiyona sahip olmanın başka yolu da dizilerden yararlanmaktır. Çokuzlunun tersine bir dizinin her elemanı aynı türden olmalıdır. Bazı dillerdeki dizilerin aksine, Rust'taki dizilerin uzunluğu sabittir. 

Bir dizinin değerlerini köşeli parantezler içine ve virgülle ayrılmış liste olarak yazarız.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-13-arrays/src/main.rs}}
```

Diziler, verilerinizin öbek yerine **stack** *(Bundan böyle yığın olarak bahsedilecektir)* üzerinde depolanmasını([Yığın ve öbek konusunu 4. bölümde inceleyeceğiz][stack-and-heap]<!-- ignore -->) veya daima belli sayıda öğelere sahip olmak istediğiniz hallerde yararlıdır. Yine de diziler, vektörler kadar esnek değildir. Standart kitaplık tarafından sağlanan vektörler diziyle benzeşen ancak boyutları değişebilen koleksiyon türleridir. Bunlardan hangisini kullanacağınızdan emin olamadığınız durumlarda olasılıkla bir vöktör türüne ihtiyacınız vardır. [Vektörleri][vectors]<!-- ignore -->  8. bölümde tartışıyor olacağız.

Diziler eleman sayısının değişmeyeceği bilinen durumlarda kullanışlıdır. Eğer ayların isimlerini kullanan bir kod yazıyor olsaydınız başka bir ayın girip çıkması mümkün olmayan ve daima 12 elemandan oluşan bir listeniz olacağından vektör yerine dizi kullanmayı tercih ederdiniz.


```rust
let aylar = ["Ocak", "Şubat", "Mart", "Nisan", "Mayıs", "Haziran",
             "Temmuz", "Ağustos", "Eylül", "Ekim", "Kasım", "Aralık"];
```

Bir dizinin türü köşeli parantez kullanılarak yazılır. Bu parantezin içinde aşağıdaki örneğe benzer biçimde, önce öğelerin türü, sonra noktalı virgül ve ardından dizide depolanacak eleman adedi belirtilir:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

Parantez içindeki `i32` depolanacak elemanların türünü, noktalı virgülden sonraki 5 rakamı ise dizinin beş öğeden oluşacağını gösterir.

Aşağıda gösterildiği gibi aynı değerlerden oluşan bir diziyi köşeli parantezlerin içine önce başlangıç değerini, ardından bir noktalı virgül ve son olarak dizide bu değerden kaç tane olacağını belirten uzunluk değerini girerek bildirebilirsiniz: 

```rust
let a = [3; 5];
```
Örnekteki `a` dizisi değeri 3 olan 5 öğeden oluşmaktadır. Bu gösterim `let a = [3, 3, 3, 3, 3];` şeklinde yazılacak kodun aynısı olup daha kısa ve özlü biçimdeki ifadesidir.

##### Dizi Öğelerine Erişim

Dizi, yığın üzerinde depolanan tek bir bellek bloğudur. Dizi öğelerine aşağıda gösterildiği gibi dizin numaralarını kullanarak erişebilirsiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-14-array-indexing/src/main.rs}}
```

örnekteki `birinci` değişkeni, dizinin indeks başlangıcı olan [0] pozisyonunda `1` değeri bulunduğundan `1` değerini, `ikinci`adındaki değişkense `[1]` pozisyonunda `2` değeri bulunduğundan `2` değerini alacaktır.

##### Geçersiz Dizi Öğesine Erişim

Dizi sınırları dışında kalan bir öğe numarasına erişmek isterseniz ne olur? 2. Bölümdeki sayı tahmin oyununa benzer bir kod kullanan aşağıdaki örneği kullanıcıdan bir dizin numarası alacak şekilde değiştirdiğimizi varsayalım: 

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access/src/main.rs}}
```

Bu kod `cargo run` komutuyla çalıştırdığınızda başarıyla derlenecektir. Program çalıştırdığınızda sizden istenilen dizin numarasını 0, 1, 2, 3, 4 olarak girerseniz o dizin numarasına karşılık gelen değer yazdırılır. Fakat dizi boyutunu aşan 5 veya 10 gibi bir değer girerseniz aşağıdaki gibi bir çıktı alırsınız:

<!-- manual-regeneration
cd listings/ch03-common-programming-concepts/no-listing-15-invalid-array-access
cargo run
10
-->

```console
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Program dizin numarasında geçersiz bir değer kullanıldığında bir çalışma zamanı hatası ve hatayı içeren bir mesaj eşliğinde sonlanarak en alt satırdaki `println!` ifadesini yürütmez. Bir öğeye dizin numarası kullanarak erişmeye çalıştığınızda Rust, belirtilen dizin numarasının dizi uzunluğundan daha küçük olup olmadığını denetler. Dizin numarası dizi uzunluğuna eşit veya büyükse programın çalıştırılması panik yoluyla sonlandırılır. Programın çalışması sırasında elde edilecek verilerin kontrol edilmesini gerektiren senaryolarda derleyicinin, kullanıcı tarafından hangi dizin numarasının girildiğine dair bir fikri olamayacağından bu tür kontrol ve denetimlerin çalışma zamanında yapılması gerekir. 

Bu senaryo Rust'ın bellek güvenliği ilkelerinin uygulamadaki örneğidir. Düşük seviyeli programlama dillerinin pek çoğunda bu tarz denetimler pek yapılmadığından  hatalı dizin numarasıyla yapılan işlem sonucu geçersiz belleğe erişilir. Ancak Rust, bellek erişimine izin vermek yerine çalışmayı durdurarak sizi bu tür hatalara karşı korur. Rust'ın [hata işleme yöntemlerine](h09-00-error-handling.html) 9. Bölümde değineceğiz.

[comparing-the-guess-to-the-secret-number]:ch02-00-guessing-game-tutorial.md#tahmin-say%C4%B1s%C4%B1n%C4%B1n-gizli-say%C4%B1-ile-kar%C5%9F%C4%B1la%C5%9Ft%C4%B1r%C4%B1lmas%C4%B1
[control-flow]: ch03-05-control-flow.html
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[stack-and-heap]: ch04-01-what-is-ownership.html#the-stack-and-the-heap
[vectors]: ch08-01-vectors.html
[unrecoverable-errors-with-panic]: ch09-01-unrecoverable-errors-with-panic.html
[wrapping]: https://doc.rust-lang.org/std/num/struct.Wrapping.html
[appendix_b]: appendix-02-operators.md
