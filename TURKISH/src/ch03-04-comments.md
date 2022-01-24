## Yorumlar

Tüm geliştiriciler kodlarının kolay anlaşılmasını isterler. Ancak bunu sağlamak için bazen kodun içine ek açıklamalar yazmak gerekir. Böyle durumlarda kaynak kodun içinde, derleyicinin görmezden geleceği fakat kodu okuyanlar için faydalı olabilecek *yorumlar* bırakılır.

Basit bir yorum örneği:

```rust
// Merhaba dünya!
```

Rust'ta klasik yorumlar iki eğik çizgiyle başlar ve satır sonuna kadar devam eder. Bir satırdan daha uzun süren yorumlar için örnekte gösterildiği gibi her satır için eğik `//` çizgi eklemeniz gerekir:

```rust
// Burada karmaşık bir şey yok. Bu kod sadece bir satıra sığmayacak kadar uzun olan
// ve alt satıra geçen yorum satırlarını örneklemek için oluşturulmuş bir kod parçasıdır.
// Kolaylıkla anlaşılabilir olduğunu umuyorum...
```

Yorumlar kodun bulunduğu satır sonuna da eklenebilirler:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Ancak bazen açıklamalanın aşağıda örneklendiği gibi açıkladığı kodun üzerindeki satırda yer aldığını göreceksiniz:

<span class="filename">Dosya adı: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rust'ın ayrıca "Bir Sandığı Cretes.io'da Yayınlamak" adlı 14. Bölümünde inceleyeceğimiz kodun belgelenmesini sağlayan başka bir yorum türü daha bulunmaktadır.