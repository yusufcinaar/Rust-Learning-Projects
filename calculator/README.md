# Rust Calculator with Enums and Pattern Matching

Bu proje, Rust programlama dilinde enum'lar ve pattern matching kullanılarak geliştirilmiş basit bir hesap makinesi uygulamasıdır.

## Özellikler

- Temel aritmetik işlemler (toplama, çıkarma, çarpma, bölme)
- Enum ve Pattern Matching kullanımı
- Kullanıcı girdisi doğrulama
- Hata kontrolü (sıfıra bölme, geçersiz girdi)

## Nasıl Çalışır

Program kullanıcıdan:
1. İlk sayıyı
2. Yapılacak işlemi (+, -, *, /)
3. İkinci sayıyı

alır ve sonucu hesaplayıp ekrana yazdırır.

## Kullanım

```bash
cargo run
```

## Teknik Detaylar

- `Operation` enum'ı ile işlemler modellendi
- Pattern matching ile işlem seçimi yapıldı
- Rust'ın güvenli bellek yönetimi prensipleri uygulandı
