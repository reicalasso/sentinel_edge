## DEĞİŞİM MODELİ — “DOSYA” DEĞİL, **EVENT**

### Temel karar

**Sentinel Edge dosya senkronize etmez.**  
**Sentinel Edge değişim senkronize eder.**

Bu cümleyi kabul etmiyorsak, proje burada biter.

---

## Değişimin atomu nedir?

### ❌ Yanlış atom

-   “Dosyanın son hali”
    
-   “Dosyayı overwrite et”
    

Bunlar:

-   conflict yaratır
    
-   veri kaybettirir
    
-   audit’i öldürür
    

---

### Doğru atom: **Change Event**

Her değişim **tekil, immutable** bir olaydır.

Örnek event:

```text
change_id      (monotonik / uuid)
peer_id
file_id        (path değil!)
parent_change  (önceki state)
timestamp
delta_payload
```

> Path = metadata  
> Kimlik = file\_id

---

## file\_id neden path değil?

Çünkü:

-   rename = path değişir ama dosya aynıdır
    
-   move = yeni dosya değildir
    
-   path güvenilmez
    

file\_id:

-   dosya ilk görüldüğünde üretilir
    
-   rename/move event üretir
    
-   içerikten bağımsızdır
    

---

## Event özellikleri (kilit kararlar)

-   **Immutable**  
    Değiştirilemez, silinmez
    
-   **Append-only**  
    Log’a eklenir
    
-   **Deterministic**  
    Aynı input → aynı delta
    
-   **Re-playable**  
    Baştan uygulanabilir
    

---

## Conflict modeli (overwrite yok)

### Conflict ne zaman olur?

-   Aynı `parent_change`
    
-   Farklı peer
    
-   Farklı delta
    

### Ne yapılır?

-   Branch oluşur
    
-   İki state de korunur
    
-   Kullanıcıya bildirilir
    

**Asla:**

-   sessiz overwrite
    
-   “son gelen kazanır”
    

---

## Zaman konusu (önemli)

-   Timestamp **karar verdirmez**
    
-   Sadece bilgi amaçlı
    

Karar:

> Nedensellik = event graph  
> Saat = sadece etiket

---

## SQLite tarafında tutulacaklar (kavramsal)

-   `files`
    
    -   `file_id`
        
    -   `current_heads`
        
-   `changes`
    
    -   `change_id`
        
    -   `file_id`
        
    -   `parent_change`
        
    -   `peer_id`
        
    -   `delta_hash`
        
    -   `created_at`
        

---

## Kararları kilitleyelim

Artık şu kararlar sabit:

-   Dosya değil, event
    
-   Overwrite yok
    
-   Conflict = branch
    
-   State = graph