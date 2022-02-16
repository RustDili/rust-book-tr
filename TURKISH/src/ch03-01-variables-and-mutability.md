## Değişkenler ve Değişkenlik

[“Değerleri Değişkenlerde Saklamak”][storing-values-with-variables]<!-- ignore --> bölümünden hatırlayacağınız üzere Rust'ta değişkenler varsayılan olarak değişmez kabul edilmekteydi. Bu kabul kodlarınızı, Rust'ın getirdiği güvenlik ve eşzamanlılık avantajlarından yararlanacak şekilde yazmanızı teşvik eden birçok Rust yaklaşımından biridir. Ancak yine de değişkenlerinizi değişebilir yapma seçeğine her zaman sahipsiniz. Şimdi gelin Rust'ın sizi değişmezliğe nasıl ve neden yönlendirdiğini ve bu değişemezliğin neden vazgeçilmez olduğunu birlikte inceleyelim.

Bir değeri değişmez olarak bildirilmiş bir değişkene bağladığınızda o değeri bir daha değiştiremezsiniz. Bu konuyu daha iyi kavrayabilmeniz için *projeler* dizininize gidip `cargo new degiskenler` komutunu kullanarak *degiskenler* adında yeni bir proje oluşturun. Ardından *degiskenler* dizinindeki *src/main.rs* dosyasını açarak içindeki kodları şu an için derlenmeyen aşağıdaki kodlarla değiştirin: 

<span class="filename">Dosya adı: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/src/main.rs}}
```

Programınızı kaydedip `cargo run` komutuyla çalıştırdığınızda aşağıdakine gibi bir hata mesajı alacaksınız:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-01-variables-are-immutable/output.txt}}
```

Bu örnek, derleyicinin programlarınızdaki hataları bulmanıza nasıl yardımcı olacağını gösterir. Deneyimli Rust geliştiricilerinin bile sık sık karşılaştığı bu sinir bozucu görünen derleyici hataları, sizin kötü programcı olduğunuzu değil, programınızın yapması gereken şeyleri henüz güvenli bir şekilde gerçekleştiremediğini anlatmaktadır.   

Hata mesajındaki `` cannot assign twice to immutable variable `x` `` uyarısı hatanın sebebi olarak **değişmez olarak bildirilen `x` değişkenine ikinci kez değer atanamayacağını** ancak bizim `x` değişkenine yeni bir değer atamaya çalıştığımızı bildirmektedir.

Değişmez olarak belirlenmiş bir değeri değiştirmeye çalışmak programda hatalara neden olabileceğinden böyle bir derleme zamanı hatası almamız önemlidir. Kodumuzun bir bölümünün bir değerin asla değişmeyeceği varsayımıyla hareket ettiği oysa başka bir bölümün bu değeri değiştirdiğini düşündüğünüzde kodun ilk bölümünün tasarlandığı gibi çalışmayacağı ortadadır. Değişken değerleri değiştirildikçe bu şekilde ortaya çıkan hataların kaynağını saptamak epey zorlaşacaktır. 
Rust'ta bir değerin değişmeyeceğini bildirdiğinizde derleyici bu değerin değişmeyeceğini garanti eder. Bu garanti değerlerin nerede ve nasıl değişeceğini dert etmeyeceğiniz anlamına geldiğinden okuma ve yazma esnasında kodun kolayca anlaşılmasını sağlayacaktır.

Ancak değişebilirliğin de pratik kod yazmak gibi güzel bir avantajı vardır. Değişkenler yalnızca varsayılan olarak değişmez olduklarından onları, tıpkı 2. Bölümde yaptığımız gibi `mut` anahtar kelimesiyle bildirerek değiştirebiliriz. İfadeye `mut`'un eklenmesi kodu anlamaya çalışanlara, buradaki değerin kodun başka bölümleri tarafından değiştirileceğini gösterir.

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

Birincisi, `mut` anahtar kelimesinin sabitler ile kullanılmasına izin verilmez. Sabitler sadece varsayılan olarak değil daima değişmez olarak kabul edilirler. Sabitleri `let` anahtar sözcüğü yerine `const` anahtar sözcüğü kullanarak bildirebilirsiniz. Bu bildirim sırasında depoladıkları değer türünü açıkça belirtmelisiniz. Türler ve tür ek açıklamaları konusuna sonraki [Veri Türleri][data-types]<!-- ignore --> başlığımızda değineceğimizden, şimdilik sabit tanımlarında türün açıkça belirtilmesi gerektiğini anımsamanız yeterlidir.

İkincisi sabitler, küresel kapsam dahil herhangi bir kapsamda bildirilebilirler. Bu da onların, kodun farklı bölümlerinde bilinen değerler olarak kullanılmasını sağlar.

Son olarak sabitler yalnızca bir işlev çağrısı sonucu olmayan sabit bir ifadeye veya çalışma zamanında hesaplanabilen başka bir değere ayarlanabilirler. 

Aşağıda bir sabit örneği yer almaktadır:

```rust
const ÜÇ_SAATTEKİ_SANİYELER: u32 = 60 * 60 * 3;
```

`ÜÇ_SAATTEKİ_SANİYELER` adlı sabit, üç saatin içinde kaç saniye olduğu bilgisini tutar. Ve değeri bir dakika içindeki saniye sayısı (60) ile bir saat içindeki dakika sayısı (60) ve saat sayısı olan (3)'ün çarpımına ayarlıdır. Rust'ın sabitler için adlandırma kuralı, kelime aralarının alt çizgi ile ayrılması ve tüm harflerin büyük olarak kullanılması şeklindedir. Derleyicinin derleme zamanında bir dizi işlemi yürütme kabiliyeti bulunduğundan, değişkenin 10,800 gibi hazır değerler yerine, anlaşılması ve doğrulaması daha kolay biçimde yazılmasına izin verilir. Sabit bildiriminde kullanılabilecek işlemler hakkında bilgilenmek için [Rust Reference bölümündeki sabit değerlendirme][const-eval] bölümünü inceleyebilirsiniz.

Sabitler, bir programın çalıştığı süre boyunca, bildirildikleri kapsam dahilinde geçerlidir. Bu durum onları, uygulamanızın farklı bölümlerinden erişilebilen, bir oyuncunun alabileceği maksimum puan sayısı veya ışık hızı gibi belirgin değerlerin bilinmesi gerektiğinde oldukça kullanışlı bir seçenek haline getirir.

Programınız genelinde kullanılan sabit olarak kodlanmış değerleri sabit olarak adlandırmak, bu değerin anlamını ileride kodun bakımını üstlenecek geliştiricilere iletmede faydalıdır. Bununla birlikte sabit olarak kodlanmış bir değerin olası bir güncelleme durumunda tek bir yerden değiştirilecek olması kod bakımı için oldukça yararlıdır.

### Gölgeleme

Bir önceki ["Tahmin Sayısının Gizli Sayı ile Karşılaştırılması"][comparing-the-guess-to-the-secret-number]<!-- ignore --> bölümünden hatırlayacağınız üzere daha önce tanımlanmış bir değişken adını kullanarak yeni bir değişken tanımlayabiliyorduk. Rust geliştiricileri tarafından "Daha önce tanımlanmış bir değişkenin sonraki tarafından gölgelendiği" şeklinde ifade edilen bu durum, değişkenin kullanılması halinde sonraki değişkene ait değerin elde edileceği anlamına gelmektedir. Aşağıdaki örnekte gösterildiği gibi, bir değişkeni aynı isim ve `let` anahtar kelimesiyle yeniden tanımlayarak gölgeleyebiliriz.

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/src/main.rs}}
```

Bu program ilk olarak `x` değişkenini 5 değerine bağlar. Ardından `let x =` ifadesini tekrarlanması sonucu `x` değişkenini, `x`'in ilk değerini alıp üzerine `1` ekleyerek `6` olacak şekilde gölgeler. Ardından gelen iç kapsamda ise `x` bir kez daha, son değerinin `2` ile çarpılaması sonucu `12` değeriyle yeniden gölgelenir. İç kapsamdan çıkıldığında içeride yapılmış olan gölgeleme de sona ereceğinden x yeniden `6` değerine döner. Program çalıştırıldığında aşağıdaki çıktıyı verecektir:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-03-shadowing/output.txt}}
```

Gölgeleme, bir değişkeni `mut` olarak işaretlemekten farklıdır. Bir değişkeni `let` anahtar kelimesi kullanmadan yeniden atamaya çalışmak derleme zamanı hatasıyla sonuçlanır. Bir değer üzerinde `let` anahtar kelimesi kullanarak bazı dönüşümler yapabiliyor olsak bile, bu dönüşümler bittiğinde değişken yine bir değişmez olarak kalacaktır.   

Gölgeleme ve `mut` arasındaki bir diğer fark ise `let` anahtar kelimesinini tekrar kullanmakla yeni bir değişken oluşturduğumuzdan, istersek değerin türünü değiştirebilir ve değişkeni aynı adla kullanmaya devam edebiliriz. Örneğin kullanıcılara gösterilecek metinler arasında boşluğun kaç karakterden oluşacağını sorduğumuzu ve gelen yanıtı sayı olarak sakladığımızı düşünelim:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-04-shadowing-can-change-types/src/main.rs:here}}
```

İlk `boşluk` değişkeni `string`, alt satırdaki birinciyle aynı adı taşıyan fakat yepyeni bir değişken olan `boşluk` değişkeniyse tam sayı türünde olduğundan bu yapıya izin verilir. Gölgeleme sayesinde `boşluk_dizgi` ve `boşluk_sayı` gibi farklı değişken isimlerine gerek duymadan, `boşluk` adını yeniden kullanarak sorunlardan kurtuluruz. Bu örnekte gölgeleme yapmak yerine aşağıda gösterildiği şekilde değişkenlikten yararlanmak isteseydik derleme zamanı hatasıyla karşılaşmış olurduk.


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/src/main.rs:here}}
```

Hata raporunda bize değişken türünün değiştirilmesine izin verilmediği bildiriliyor. 

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-05-mut-cant-change-types/output.txt}}
```

Artık değişkenlerin nasıl çalıştığını anladığımıza göre depolayabilecekleri veri türlerini inceleyebiliriz. 


[comparing-the-guess-to-the-secret-number]:
ch02-00-guessing-game-tutorial.html#tahmin-sayisinin-gizli-sayi-ile-karsilastirilmasi
[data-types]: ch03-02-data-types.html#veri-turleri
[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#degerleri-degiskenlerde-saklamak
[const-eval]: https://doc.rust-lang.org/reference/const_eval.html
