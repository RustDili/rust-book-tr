## İşlevler

İşlevler Rust kodlarında yaygın olarak kullanılır. Dildeki en önemli işlevlerden biri olan ve programın giriş noktasını oluşturan `main` ile, yeni bir işlev bildirmeye yarayan `fn` anahtar sözcüğünü daha önce görmüştünüz.

Rust geleneksel olarak değişken ve işlev isimlerinde küçük harflerden oluşan ve ayrı kelimelerin alt çizgi ile birbirine bağlandığı *snake_case* tarzını kullanmaktadır.

Aşağıdaki programda bir işlev tanımı örneklenmektedir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```
Rust'ta bir işlevi `fn` anahtar kelimesi ardından işlev adı ve bir parantez seti ile tanımlarız. Süslü parantezlerin konumu derleyiciye işlevin nerede başlayıp nerede bittiğini bildirir.

Halihazırda tanımlı olan bir işlev ise adı ve arkasına gelen parantez seti ile çağırılır. Örneğimizde `başka_işlev` zaten tanımlı olduğundan `main` işlevi içinden çağırılabilmektedir. Kaynak kodumuzdaki `başka_işlev`'i `main` işlevinden *sonra* tanımladığımıza dikkat edin. Rust işlevlerin nerede tanımlandığıyla ilgilenmediğinden dilerseniz işlevlerinizi `main` işlevinden önce de tanımlayabilirsiniz.

İşlevlere daha yakından bakabilmek için cargo new komutu kullanarak projeler dizininde *islevler* adlı yeni bir proje başlatın. Arkasından `başka_işlev` örneğini *src/main.rs*  içine alarak kodunuzu çalıştırın. Ekranınızda aşağıdaki çıktıyı görnelisiniz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Satırlar `main` işlevinde göründüğü sırayla işletilir. İlk olarak "Merhaba dünya!" mesajı, arkasından `başka_işlev` çağrısının ürettiği "Bir başka işlev." mesajı yazdırılır. 

### Parametreler

İşlevleri, işlev imzasına ait özel değişkenler olan parametreler ile birlikte tanımlayabiliriz. Bir işlevde parametreler bulunuyorsa bu parametrelere somut değerler iletebilirsiniz. İşlevlere parametre olarak iletilen somut değerler argüman aolarak adlandırılır. Fakat insanlar konuşmalarında bu kavramları kullanırken, işlev tanımındaki değişkenleri anlatan *parametre* yerine argüman, işlev çağrısı esnasında iletilen somut değerleri temsil eden *argüman* yerine parametre olarak bahsetmekte veya tam tersi biçimde birbirinin yerine geçirerek kullanma eğilimindedirler. 

Aşağıda `başka_işlev`'in şimdiki sürümüne bir parametre ekliyoruz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Programı çalıştırdığınızda aşağıdaki çıktıyı alıyor olmalısınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

Artık `başka_işlev`'in `x` adında ve `i32` türünde bir parametresi vardır. Bu işleve 5 değeri iletildiğinde `println!` makrosu bu değeri biçimlendirme dizgisindeki süslü parentezlerin olduğu yere koyar.

İşlev imzalarında bulunan her parametrenin türü *mutlaka* bildirilmelidir. Bu Rust tasarlanırken alınan bilinçli bir karardır. İşlev  tanımlarında tür bildirimi zorunluluğu, derleyicinin kullanılacak türü kodun başka bir yerinde kullanılmadan anlamasını sağlar.

Tanımlanan parametre sayısı birden fazlaysa bildirimlerin arası aşağıdaki gibi virgül ile ayrılmalıdır:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Bu örnek iki parametresi bulunan `etiket_değerlerini_yazdır` adlında bir işlev oluşturur. İşlevin `değer` adındaki ilk parametresi `i32`, `birim` adındaki ikinci parametresiyse `char` türündedir. Bu işlev `değer` ve `birim` verilerini içeren bir metin yazdırır. 

Bu kodu islevler adlı projenizin *src/main.rs* dosyasında bulunan bir önceki kodla değiştirdikten sonra `cargo run` komutunu kullanarak çalıştırın:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

İşlevi `değer` verisi için `5`, `birim` verisi için `h` ile çağırdığımızdan dolayı program çıktısı bu değerleri içermektedir. 

### Deyimler ve İfadeler
<!-- 
Ç.N: 
Statement: değerli ifade anlamında 
Expression: emire daha yakın ifade anlamında
statementler değer taşırken expression bir iş yapar.
-->

İşlev gövdeleri isteğe bağlı olarak bir ifadeyle biten deyimlerden oluşur. Her ne kadar şu ana kadar gördüğünüz işlevler bitiş ifadesi içermiyor olsa da deyimin bir parçası olan ifadeyle karşılaştınız. Rust ifade tabanlı bir dil olduğundan bu ayrıntının anlaşılması önemlidir. Bu ayrım diğer dillerde olmadığından deyim ve ifadenin ne olduğuna ve farklarının işlev gövdelerini nasıl etkilediğini inceleyelim.

*Deyimler* bazı eylemleri gerçekleştiren ve bir değer döndürmeyen talimatlarken, *ifadeler* sonuç olarak bir değer döndürürler. 

Zaten deyim ve ifadeleri daha önce kullanmıştık. Örneğin `let` anahtar sözcüğüyle değişken oluşturarak ona değer atamak ve Örnek 3-1'deki `let y = 6` talimatı birer deyimdir.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

<span class="caption"> Örnek 3-1: Deyimden içeren bir `main` işlevi</span>

Tıpkı bu örneğin tamamı gibi işlev tanımları da kendi içinde birer deyimdir.

Aşağıdaki kodda yapıldığı gibi `let` deyimini bir başka değişkene atamaya kalktığınızda, deyimler değer döndürmedikllerinden hata almanız kaçınılmazdır:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Bu programı çalıştırdığınızda aşağıdaki gibi bir hata alacaksınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

Burada `let y = 6` deyimi bir değer döndürmeyeceğinden `x`'in bağlanacağı bir değer yoktur. Bu durum böyle bir atamanın atanan değeri döndürdüğü C ve Ruby gibi dillerden farklıdır. Bahsedilen dillerde `x = y = 6` şeklide bir talimatla hem `x` hem de `y` değişkenlerine `6` değerini atayabilirsiniz. Ancak Rust'ta durum böyle değildir.

İfadeler ise değer olarak hesaplanır ve Rust'ta yazacağınız kodların çoğunluğu ifadelerden oluşacaktır. Bunu `11` sonucunu veren `5 + 6` matematiksel işlemiymiş gibi düşünün. İfadeler deyimlerin bir parçası olabilir. Örnek 3-1'de yer alan `let y = 6` deyimindeki `6`'nın, işletildiğinde `6` olarak değerlendirilmesi gibi işlev çağrıları da birer ifadedir. Tıpkı makro çağrılarının birer ifade olması gibi süslü parantezlerle oluşturulan kapsam blokları da birer ifadedir. Örneğin: 

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

Burada:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

`let` deyiminin parçası olan ve `4` olarak değerlendirilen blok bir ifadedir. Bu değer `let` deyiminin bir parçası olduğundan `y` değişkenine bağlanır. Şimdiye kadar gördüğünüz çoğu satırın aksine `x + 1` satırının sonunda noktalı virgülün olmadığına dikkat edin. İfadeler noktalı virgül ile sonlanmaz. Eğer bu ifadenin sonuna noktalı virgül eklerseniz onu değer döndürmeyen bir deyime dönüştürürsünüz. Bunu, bir sonraki konuda dönüş değerleri ve ifadeleri incelerken aklınızda bulundurun.

### Değer Döndüren İşlevler

İşlevler kendilerini çağıran koda değer döndürebilirler. Dönüş değerleri isimlendilmez fakat türleri bir oku (`->`) takiben bildirilir. Rust'ta işlevin dönüş değeri, işlev gövdesindeki son ifadenin değeriyle aynıdır. İşlevden erken çıkabilmek için dönüş değeri eşliğinde `return` anahtar sözcüğünü kullanabilirsiniz, ancak pekçok işlev son ifadeyi örtük biçimde döndürür. Aşağıda değer döndüren bir işlev örneği verilmektedir:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

`Beş` işlevinde `5` rakamı hariç hiçbir işlev çağrısı, makro ve `let` deyimi yoktur. Ve bu Rust'ta tamamen geçerli bir işlevdir. İşlev dönüş türünün `-> i32` olarak belirtildiğine dikkat edin. Bu kodu çalıştırdığınızda aşağıdaki gibi çıktı üretmelidir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

İşlevin dönüş değeri `5` olduğundan dönüş türü de `i32` olarak ayarlanmıştır. İşlevi dikkatle incelediğimizde iki şeyle karşılaşırız. İlki olarak `let x = beş();` satırı, `x` değişkenini başlatmak için işlevin dönüş değerinden yararlandığımızı anlatır. `Beş` işlevi `5` değerini döndürdüğünden o satır aşağıdakiyle aynı anlama gelir.

```rust
let x = 5;
```

İkinci olarak, `beş` işlevi parametresiz olmamasına rağmen döndürülecek değerin türünü tanımlar. Bununla birlikte işlev gövdesi döndürmek istediğimiz değeri ifade ettiğinden noktalı virgül olmadan tek bir `5`'ten oluşur.

Başka bir örneği inceleyelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Bu kodu çalıştırdığınızda `X'in değeri: 6` sonucunu yazdıracaktır. Ama `x + 1`'in bulunduğu satır sonuna onu bir ifadeden deyime çeviren noktalı virgül eklerseniz hata ile karşılaşırsınız:

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Bu kod derlendiğinde aşağıdaki benzer şekilde hata üretecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

Hatayı özetleyen “mismatched types” (uyumsuz türler) mesajı bize bu kodunun temel sorununu göstermektedir. `artı_bir` işlev tanımı `i32` türünde bir değer döndürüleceğini bildirirken artık bir deyim olan ve birim türü `()` olarak temsil edilen satır bir değer olarak değerlendirilemez. Bu nedenle işlev tanımıyla çelişen ve bir hatayla sonuçlabilecek hiçbir şey döndürülmez. Rust hata raporunda noktalı virgülün kaldırılmasını öneren bir mesajla sorunun çözülmesine yardım eder.