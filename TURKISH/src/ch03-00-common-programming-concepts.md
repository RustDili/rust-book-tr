# Ortak Programlama Kavramları

Bu bölüm neredeyse tüm programlama dillerinde görülen kavramları ve bu kavramların Rust'ta nasıl çalıştığını kapsar. Pek çok dilin özünde birbiriyle ortak noktalar olduğundan bu bölümde sunulan kavramların hiçbiri Rust'a özgü değildir. Ancak biz bu kavramları Rust bağlamında tartışacak ve kullanımlarıyla ilgili kuralları açıklayacağız.

Özellikle değişkenler, temel türler, işlevler, yorum ve kontrol akışı hakkında bilgi edineceksiniz. Henüz yolun başındayken Rust programlama alt yapısını oluşturan bu bilgilerin öğrenilmesi size sağlam bir temel kazandıracaktır.

> #### Anahtar Kelimeler
>
> Diğer programlama dillerinde olduğu gibi Rust dilinde de, yalnızca dilin kullanabileceği bir dizi anahtar
> kelime vardır. Bu kelimeleri değişken veya işlev adlarında kullanamazsınız. Çoğu anahtar kelime özel bir anlam taşıdığından
> Rust programlarındaki çeşitli görevlerde bu kelimelerden faydalanacaksınız. Her ne kadar bu kelimelerden bazıları şu an için 
> işlevsel değilmiş gibi görünse de, bunlar yakın gelecekte Rust'a eklenmesi planlanan işlevler için ayrılmıştır. Bu anahtar
> kelimelerin listesi kitabınızın [Ek A][appendix_a]<!-- ignore --> bölümünde bulunmaktadır.

[appendix_a]: appendix-01-keywords.md