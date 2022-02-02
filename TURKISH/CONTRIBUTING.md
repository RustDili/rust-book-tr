# Çevirilere nasıl katılabilirim?

## Rehber

### Çevirilecek Terimler

Bazı temel İngilizce teknik terimlerin Türkçe karşılıklarını sunmamız gerekiyor. Çevirileriniz sırasında karşılaştığınız terimlerin Türkçe karşılığını öncelikle bu tabloda arayınız. [Terimin Türkçe karşılığı tabloda yoksa yahut daha iyi bir öneriniz varsa bunu bizimle paylaşabilirsiniz.](https://github.com/RustDili/rust-book-tr/discussions/2)

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

### Çeviri Akışı

01. Üzerinde çalıştığınız bölüm başlığını [Çalışılmakta olan bölümlere](https://github.com/RustDili/rust-book-tr/issues/3) ekleyin. 
  - Çalışmak için önceden bildirilmiş ve üzerinde aktif olarak çalışılan bölümler yerine farklı bir bölüm seçin.
  - Eğer [rust-book-tr]() reposunu edinmemişseniz kendi hesabınız üzerinden repoyu çatallayın.

02. Ardından `git clone https://github.com/<çatal_adınız>.git` biçimini kullanarak reponuzu yerel çalışma ortamınıza aktarın.

03. `git checkout -b <kendi-dalınızı>` komutuyla çalışmak istediğiniz bölüm için kendi dalınızı oluşturun.

04. Çalışmak istediğiniz dosyayı `TURKISH/src/` kopyalayayın.
  - Çalıştığınız bölümün bağlantısını `TURKISH/src/SUMMARY.md` ve `TURKISH/src/README.md` dosyalarına ekleyin.    

05. Çevirinizi asıl metnin paragraf yapısına uygun biçimde yürütmeniz daha sonraki güncellemere kolayca adapte olmanızı sağlar. Bu nedenle çevirinizi İngilizce aslındaki yapıya bağlı kalarak sürdürmeye özen gösterin.

06. Gözden geçirmelerinizi tamamladıktan sonra `git add -A && git commit -m "Yaptığınız çalışmayı açıklayın"` komutuyla taahhüdünüzü yapın.
  - Mümkün olduğunca tüm taahhütlerinizi bir taahhütte toplayın

07. `git push origin` komutuyla çalışmanızı uzak deponuza gönderin. 

08. Çalışmanızın gözden geçirilip çevirilere eklenmesi için Github üzerinden çekme talebi oluşturun.

09. Çalışmanız gözden geçirildikten yahut gerekli düzeltmelerden sonra çeviri diziniyle birleştirilecek ve issue üzerindeki etiketi tamamlandıya ayarlanacaktır.   
