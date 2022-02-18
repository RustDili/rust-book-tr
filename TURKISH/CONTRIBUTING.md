# Çevirilere nasıl katılabilirim?

## Rehber

### Çevirilecek Terimler

Bazı temel İngilizce teknik terimlerin Türkçe karşılıklarını sunmamız gerekiyor. Çevirileriniz sırasında karşılaştığınız terimlerin Türkçe karşılığını öncelikle [bu tabloda](sozluk)<!--ignore --> arayınız. Terimin Türkçe karşılığı tabloda yoksa yahut daha iyi bir öneriniz varsa bunu [bizimle paylaşabilirsiniz.](https://github.com/RustDili/rust-book-tr/discussions/2)

### Rust Kodlarının Çevrilmesi

Rust kodlarında çevrilmesi gereken bölümler aşağıdaki gibidir:

- Yorum satırları
- Metin halindeki mesajlar (Dizgi-String)
- Değişken, işlev ve yapı isimleri gibi özelleştirilebilecek isimler Rust adlandırma kuralına uygun biçimde `değişken_ismi` şeklinde ve Türkçe olmalıdır.
- Kodun ana yapısı ve terminal çıktıları olduğu gibi bırakılmalıdır.

### Dosyalar

Dosya isimleri çevrilmeyip olduğu gibi bırakılacaktır. Çevirdiğiniz dosyanın her satırının boşluklar dahil en fazla 80-160 karakter veya veya `soft wrap` biçiminde olmasına özen gösterin. 

### İmla Kuralları 
- Çeviri sırasında anlamı oturtabilmek kolay değildir. Ancak bu dil bilgisi kuralları ve noktalama işaretlerine uymamıza engel değildir. Olabildiğince temiz ve özenli bir çeviri yapmak temel hedefimizdir. Dahi anlamındaki *-de*, bağlaç olan *-ki* "ve, veya, ya da" kavramlarını doğru kullanmamız önemlidir.

### Çeviriler Hakkında
Bu kitabın çevirisi bölümler halinde yapılmaktadır. Bölümlerden kastımız sayfalar değil, kitabın ana başlıklarını oluşturan örneğin 3. Bölüm için `ch03-00-bölüm-adı`'ndan başlayan ve `ch04-00-bölüm-adı`'na kadar olan tüm sayfalardır. Çeviri katkısında bulunmak isteyen çevirmen adayları en az bir bölümün çevirisini üstlenmek zorundadır. Tam bir bölüm halinde olmayan, birkaç satır veya birkaç paragraftan yahut tamamlanmamış bir bölümden oluşan çekme isteklerine yanıt verilmeyecektir.

Çevirmen adayı çevirmek istediği bölüm hakkında görevlendirme olup olmadığını [Çalışılmakta olan bölümler](https://github.com/RustDili/rust-book-tr/issues/3) alanından inceleyip üzerinde çalışılmayan bir bölümün çevirisine aday olabilir. Rust dili ekibi haricindeki çevirmen adayları sadece bir bölümün çevrilmesine aday olabilirler. Çevirmen adayları çevirecekleri bölümü 15 gün içinde yapacakları çekme isteğiyle kontrole açmalıdırlar. Bu süre içinde kontrole açılmayan bölümler çevrilmemiş kabul edilecek ve çevirileri başka çevirmen adaylarına sunulabilecektir.

Çevrilecek bölüm başlıkları [Çalışılmakta olan bölümler](https://github.com/RustDili/rust-book-tr/issues/3) başlığı altında bildirilmedikçe çevirileri yapılmış bile olsa çekme talepleri kabul edilmeyecektir. Lütfen çalışmak istediğiniz bölüm/bölümleri diğer çevirmen arkadaşların çeviri takibini kolaylaştırmak amacıyla açtığımız bu başlığa bildirin.

Pek çok çeviri işinde olduğu gibi Rust dilinin çevirisinde de motamot çeviri yaklaşımı çoğu zaman anlatılanı bağlamından koparmaktadır. Ekip olarak böyle durumlarla karşılaştığımızda bilginin özüne sadık kalarak, anlam bütünlüğünü bozmayacak ve kitabın geri kalanıyla uyumlu olacak şekilde düzenlemeler yapmaktayız. "Orada demek istenilen şey" hakkında epey düşünmüş olduğumuzu ve bu tür düzeltme taleplerine yanıt vermeyeceğimizi bilmenizi isteriz. 

Rust dili ekibi tarafından çevirisi tamamlanan bölümler, bir diğer çevirmen tarafından çapraz kontrol sürecine alınar. Bu süreçte imla, terminoloji, cümle bozuklukları, anlatım uyumu, anlam bütünlüğü gibi bir dizi denetimler yapılır. Bu denetim sürekli olduğundan bu tür sorunlarla karşılaşmanız halinde bunu düzelteceğimizi bilmenizi ve bunlarla ilgili düzeltme önerisi çekme, isteği gibi taleplerle gelmemenizi isteriz. 

Rust yaşayan bir programlama dili olduğundan ve sürekli güncellendiğinden, çevirisi tamamlanmış bölümlere gelen revizyonlar ara sıra gecikse de mutlaka yansıtılır. Azıcık sabır...

#### Çeviri Akışı

01. Üzerinde çalıştığınız bölüm başlığını [Çalışılmakta olan bölümlere](https://github.com/RustDili/rust-book-tr/issues/3) ekleyin. 
  - Çalışmak için önceden bildirilmiş ve üzerinde aktif olarak çalışılan bölümler yerine farklı bir bölüm seçin.
  - Eğer [rust-book-tr](https://github.com/RustDili/rust-book-tr) reposunu edinmemişseniz kendi hesabınız üzerinden repoyu çatallayın.

02. Ardından `git clone https://github.com/<çatal_adınız>.git` biçimini kullanarak reponuzu yerel çalışma ortamınıza aktarın.

03. `git checkout -b <kendi-dalınızı>` komutuyla çalışmak istediğiniz bölüm için kendi dalınızı oluşturun.

04. Çalışmak istediğiniz dosyayı `TURKISH/src/` kopyalayayın.
  - Çalıştığınız bölümün bağlantısını `TURKISH/src/SUMMARY.md` ve `TURKISH/src/README.md` dosyalarına ekleyin.    

05. Çevirinizi asıl metnin paragraf yapısına uygun biçimde yürütmeniz daha sonraki güncellemere kolayca adapte olmanızı sağlar. Bu nedenle çevirinizi İngilizce aslındaki yapıya bağlı kalarak sürdürmeye özen gösterin.

06. Gözden geçirmelerinizi tamamladıktan sonra `git add -A && git commit -m "Yaptığınız çalışmayı açıklayın"` komutuyla taahhüdünüzü yapın.
  - Mümkün olduğunca taahhütlerinizin tamamını bir taahhütte toplayın.

07. `git push origin` komutuyla çalışmanızı uzak deponuza gönderin. 

08. Çalışmanızın gözden geçirilip çevirilere eklenmesi için Github üzerinden çekme talebi oluşturun.

09. Çalışmanız gözden geçirildikten yahut gerekli düzeltmelerden sonra çeviri diziniyle birleştirilecek ve issue üzerindeki etiketi tamamlandıya ayarlanacaktır.   
