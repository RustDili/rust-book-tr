## Değişkenler ve Değişkenlik

[“Değerleri Değişkenlerde Saklamak”][storing-values-with-variables]<!-- ignore --> bölümünden hatırlayacağınız üzere Rust'ta değişkenler varsayılan olarak değişmez kabul edilmekteydi. Bu kabul kodlarınızı, Rust'ın getirdiği güvenlik ve eşzamanlılık avantajlarından yararlanacak şekilde yazmanızı teşvik eden birçok Rust yaklaşımından biridir. Ancak yine de değişkenlerinizi değişebilir yapma seçeğine her zaman sahipsiniz. Şimdi gelin Rust'ın sizi değişmezliğe nasıl ve neden yönlendirdiğini ve bazen bu değişemezlikten neden vazgeçmemiz gerektiğini birlikte inceleyelim.

Bir değişmez haldeki bir değişkene isim verilerek değer atandığında o değişkenin değerini artık değiştiremezsiniz. Bu konuya açıklık getirebilmek için projeler dizininde `cargo new degiskenler` komutunu kullanarak *degiskenler* adında yeni bir proje oluşturalım. Ardından *degiskenler* dizinindekki *src/main.rs* dosyasını açarak içindeki kodları şu an için derlenmeyen aşağıdaki kodlarla değiştirelim: 

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Programınızı kaydedip `cargo run` komutuyla çalıştırdığınızda aşağıdakine gibi bir hata mesajı alacaksınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Bu örnek, derleyicinin programlarınızdaki hataları bulmanıza nasıl yardımcı olacağını gösterir. Deneyimli Rust geliştiricilerinin bile karşılaşmaktan muaf olmadığı bu sinir bozucu görünen derleyici hataları, sizin kötü programcı olduğunuzu değil, programınızın yapması gereken şeyleri henüz güvenli bir şekilde gerçekleştiremediğini söylemektedir.   

Hata mesajındaki `` cannot assign twice to immutable variable `x` `` uyarısı hatanın sebebi olarak **değişmez olarak bildirilen `x` değişkenine ikinci kez değer atanamayacağını** ancak bizim `x` değişkenine yeni bir değer atamaya çalıştığımızı bildirmektedir.

Değişmez olarak belirlenmiş bir değeri değiştirmeye çalışmak programda hatalara neden olabileceğinden böyle bir derleme zamanı hatası almamız önemlidir. Kodumuzun bir bölümünün bir değerin asla değişmeyeceği varsayımıyla hareket ettiği oysa başka bir bölümün bu değeri değiştirdiğini düşündüğünüzde kodun ilk bölümünün tasarlandığı gibi çalışmayacağı ortadadır. Bu şekilde ortaya çıkan hataların kaynağını saptamak, değişken değeri ara sıra değiştirildiğinde daha da çok zorlaşır. 
Rust'ta bir değerin değişmeyeceğini bildirdiğinizde derleyici bu değerin değişmeyeceğini garanti eder. Bu garanti bir kodu okur veya yazarken, değerlerin nerede ve nasıl değişeceğini takip etmenize gerek olmadığı anlamına gelmekte ve kodlarınızın kolayca anlaşılmasını sağlamaktadır.

Ancak değişebilirlik pratik kod yazmak gibi çok sayıda fayda sağlar. Değişkenler yalnızca varsayılan olarak değişmez olduklarından, tıpkı 2. Bölümde yaptığımız gibi önlerine `mut` kelimesini ekleyerek onları değişebilirsinizr. Anahtar kelime `mut`'un eklenmesi ileride bu kodu okuyacaklara, bu değişken değerinin kodun diğer bölümleri tarafından değiştirileceğini de gösterir.

Örneğin *src/main.rs* dosyasını aşağıdaki şekilde değiştirelim:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/src/main.rs}}
```

Programı bu şekilde çalıştırdığımızda aşağıdaki çıktıyı elde ederiz:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-02-adding-mut/output.txt}}
```

`mut`'u kullanmakla `x`'e bağlı olan `5` değerinin `6` olarak değiştirilmesine izin vermiş oluruz. Hatalardan kaçınmanın yanı sıra verilmesi gereken başka tavizler de vardır. Örneğin, büyük yapılarla çalışırken mevcut bir örneği yerinde değiştirmek, yeni atanacak örneklerin kopyalanarak geri döndürülmesinden daha hızlı olabilir. Yahut küçük veri yapılarıyla çalışılırken yeni örnekler oluşturmak ve işlevsel programlama tarzından daha fazla yararlanmak anlaşılabilirliği arttıracağından, bu netlik uğruna performanstan ödün vermek göze alınabilecek bir tercih olabilir.

### Sabitler

Değişmez değişkenler gibi sabitler de bir isme bağlı olan ve değiştirilmesine izin verilmeyen değerlerdir, ancak sabitler ve değişkenler arasında bazı farklılıklar bulunur.

Birincisi, `mut` anahtar kelimesinin sabitler ile kullanılmasına izin verilmez. Sabitler sadece varsayılan olarak değil daima değişmez olarak kabul edilirler. Sabitleri `let` anahtar sözcüğü yerine `const` anahtar sözcüğü kullanarak bildirebilirsiniz. Bu bildirim sırasında depoladıkları değer türünü açıkça belirtmelisiniz. Türler ve tür ek açıklamaları konusunu bir sonraki konumuz olan [Veri Türleri][data-types]<!-- ignore --> bölümünde inceleyeceğimizden bu konunun ayrıntıları hakkında endişelenmeniz gerekmez. Şimdilik sabitleri bildirdiğiniz esnada türün açıkça belirtilmesi gerektiğini anımsamanız yeterlidir.

İkincisi sabitler, küresel kapsam dahil herhangi bir kapsamda bildirilebilirler. Bu da onların, kodun farklı bölümlerinde bilinen değerler olarak kullanılmasını sağlar.

Son olarak sabitler yalnızca bir işlev çağrısı sonucu olmayan sabit bir ifadeye veya çalışma zamanında hesaplanabilen başka bir değere ayarlanabilirler. 

Aşağıda bir sabit örneği yer almaktadır:

```rust
const ÜÇ_SAATTEKİ_SANİYELER: u32 = 60 * 60 * 3;
```

`ÜÇ_SAATTEKİ_SANİYELER` adlı sabit üç saatin içinde kaç saniye olduğu bilgisini tutar. Ve değeri bir dakia içindeki saniye sayısı (60) ile bir saat içindeki dakika sayısı (60) ve saat sayısı olan (3)'ün çarpımına ayarlıdır. Rust'ın sabitler için adlandırma kuralı, kelimele aralarının alt çizgi ile ayrılması ve tüm harflerin büyük olarak kullanılmasıdır. Derleyici derleme zamanında bir dizi işlemi değerlendirebileceğinden, değerin doğrudan 10,800 olarak ayarlanması yerine, anlaşılması ve doğrulaması daha kolay olan biçimde yazılmasına izin verir. Sabit bildiriminde kullanılabilecek işlemler hakkında bilgilenmek için [Rust Reference bölümündeki sabit değerlendirme][const-eval] bölümünü inceleyebilirsiniz.

Sabitler, bir programın çalıştığı süre boyunca, bildirildikleri kapsam dahilinde geçerlidir. Bu durum onları, uygulamanızın farklı bölümlerinden erişilebilen, bir oyuncunun alabileceği maksimum puan sayısı veya ışık hızı gibi belirgin değerlerin bilinmesi gerektiğinde oldukça kullanışlı bir seçenek haline getirir.

Programınız genelinde kullanılan sabit olarak kodlanmış değerleri sabit olarak adlandırmak, bu değerin anlamını ileride kodun bakımını üstlenecek geliştiricilere iletmede faydalıdır. Bununla birlikte sabit olarak kodlanmış bir değerin olası bir güncelleme durumunda tek bir yerden değiştirilecek olması kod bakımı için oldukça yararlıdır.

### Gölgeleme

Bir önceki ["Tahmin Sayısının Gizli Sayı ile Karşılaştırılması"][comparing-the-guess-to-the-secret-number]<!-- ignore --> bölümünden hatırlayacağınız üzere daha önce tanımlanmış bir değişken adıyla yeni bir değişken tanımlayabilirsiniz. Rust geliştiricileri tarafından önce tanımlanan değişkenin sonraki tarafından gölgelendiği ifade edilen bu durum, değişkenin kullanılması halinde ikinci değişkene ait değerin elde edileceği anlamına gelmektedir. Aşağıdaki örnekte gösterildiği gibi, bir değişkeni aynı isimle ve `let` anahtar kelimesi tekrar kullanarak gölgeleyebiliriz.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Bu program ilk olarak `x` değişkenini 5 değerine bağlar. Ardından `let x =` ifadesini tekrarlanması sonucu `x` değişkenini, `x`'in orijinal değerini alıp üzerine `1` ekleyerek `6` olacak şekilde gölgeler. Ardından gelen iç kapsamda ise değer üçüncü kez gölgelenerek önceki değer `2` ile çarpılır ve `x` değişkeni `12` değerini almış olur. İç kapsamdan çıkıldığında içeride yapılmış olan gölgeleme de sona ereceğinden x yeniden `6` değerine döner. Program çalıştırıldığında aşağıdaki çıktıyı verecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Gölgeleme, bir değişkeni `mut` olarak işaretlemekten farklıdır. Bir değişkeni `let` anahtar kelimesi kullanmadan yeniden atamaya çalışmak derleme zamanı hatasıyla sonuçlanır. Bir değer üzerinde `let` anahtar kelimesi kullanarak bazı dönüşümler yapabiliyor olsak bile, bu dönüşümler bittiğinde değişken yine bir değişmez olarak kalacaktır.   

Gölgeleme ve `mut` arasındaki bir diğer fark ise `let` anahtar kelimesini tekrar kullanmakla etkili bir şekilde yeni bir değişken oluşturduğumuzdan, değerin türünü değiştirebilir ve değişkeni aynı adla kullanmaya devam edebiliriz. Örneğin kullanıcılara gösterilecek metinler arasında kaç karakterlik boşluk görmek istediklerini sorduğumuzu ve yanıtı sayı olarak saklamak istediğimizi düşünelim:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

İlk `boşluk` değişkeni `string` (dizgi), alt satırdaki birinciyle aynı adı taşıyan fakat yepyeni bir değişken olan `boşluk` değişkeniyse tam sayı türünde olduğundan bu yapıya izin verilir. Gölgelemenin bu avantajı sayesinde `boşluk_dizgi` ve `boşluk_sayı` gibi farklı değişkenler oluşturmadan, `boşluk` adını tekrar kullanmakla bu sorunlardan kurtuluvermiş oluruz. Eğer bunun yerine `mut` anahtar sözcüğünü aşağıdaki gibi kullanmaya kalkarsak bir derleme zamanı hatası alırız.


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Hata bize bir değişken türünün değiştirilmesine izin verilmediğini bildiriyor. 

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Artık değişkenlerin nasıl çalıştığını anladığımıza göre alabilecekleri veri türlerini inceleyebiliriz. 


[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#tahmin-sayisinin-gizli-sayi-ile-karsilastirilmasi
[data-types]: ch03-02-data-types.html#veri-turleri
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#degerleri-degiskenlerde-saklamak
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
