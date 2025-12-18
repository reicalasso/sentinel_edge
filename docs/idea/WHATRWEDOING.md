## 1️⃣ PROBLEM TANIMI (kilit taş)

### ❌ Bugünkü gerçek durum

Şu an piyasadaki çözümlerden biriyle çalışan bir teknik kullanıcı şunları yaşıyor:

-   Dosya senkronizasyonu **merkezi servislere bağımlı**
    
-   Gerçek zamanlı değişimler:
    
    -   ya **tam dosya** gönderiyor
        
    -   ya da **gecikmeli**
        
-   Peer’lar arası güven:
    
    -   ya “herkese açık”
        
    -   ya da tamamen kapalı
        
-   Güvenlik:
    
    -   şifreleme var ama **davranış farkındalığı yok**
        
-   Anomali:
    
    -   dosya *içeriği* taranıyor
        
    -   ama **davranışsal sapmalar** görülmüyor
        
-   Offline → online dönüş:
    
    -   overwrite
        
    -   veri kaybı
        
    -   sessiz conflict
        

Bu **gerçek bir problem**. Özellikle:

-   homelab
    
-   dev ortamı
    
-   edge cihazlar
    
-   güvene takıntılı kullanıcılar
    

---

## ✅ Sentinel Edge’in çözdüğü problem (net)

> **Sentinel Edge, güvenilir peer’lar arasında dosya değişimlerini gerçek zamanlı, overwrite etmeden, davranış bazlı güvenlik farkındalığıyla senkronize eden bir P2P dosya evrimi sistemidir.**

Burada 3 kritik kelime var:

-   **değişim**
    
-   **güvenilir peer**
    
-   **davranış**
    

---

## 🚫 Özellikle çözmediği problemler

Bunu yazmak zorundayız.

Sentinel Edge:

-   ❌ public cloud storage değil
    
-   ❌ dosya paylaşım platformu değil
    
-   ❌ virüs tarayıcı değil
    
-   ❌ backup ürünü değil (yan ürün olabilir)
    

Bu sınırlar çizilmezse proje ölür.

---

## 🎯 Hedeflenen kullanıcı (problemle birebir örtüşen)

-   2–20 arası cihaz
    
-   Teknik kullanıcı
    
-   Kendi altyapısına sahip
    
-   “Dosyam nerede?” sorusunu önemseyen
    
-   Buluta **mecbur kalmak istemeyen**
    

Bu kişi:

-   performans ister
    
-   kontrol ister
    
-   sessiz overwrite’ten nefret eder
    

---

## 🧠 Problem cümlesinin test kriteri

Şu soruya **net evet** diyebiliyor muyuz?

> “Bugün iki cihazım aynı dosyayı farklı şekilde değiştirirse,  
> Sentinel Edge **veri kaybı olmadan**, **bunu fark ederek** ve **güvenli biçimde** yönetebilir mi?”

Eğer evet → doğru problem  
Eğer hayır → fikir daha oturmamış

---

## 🔒 Bu adımı kilitliyoruz
