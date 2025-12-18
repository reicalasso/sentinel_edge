## ML TABANLI ANOMALİ TARAMASI

### Amaç

-   Dosya içeriğini taramak değil
    
-   **Peer davranışlarını izlemek**
    
-   Anormal değişim / potansiyel riskleri tespit etmek
    
-   Sistem güvenliğini artırmak
    

---

## Hangi sinyaller ML’ye gider?

1.  **Delta frekansı**
    
    -   Bir dosya kısa sürede çok kez değişti mi?
        
    -   Peer normalden hızlı mı yolluyor?
        
2.  **Dosya boyutu / block pattern**
    
    -   Normalde küçük dosya aniden çok büyük delta üretiyor mu?
        
    -   Binary dosya → text gibi davranıyor mu?
        
3.  **Peer davranış trendleri**
    
    -   Yeni peer sürekli overwrite / branch üretiyor mu?
        
    -   Offline → online dönüşlerinde anormal pattern?
        
4.  **Entropy değişimi**
    
    -   Dosya içeriği aniden yüksek entropili hale geldi mi?
        
    -   Olası şifreleme / malware belirtisi
        

---

## MVP yaklaşımı

-   Basit ML model:
    
    -   Threshold-based scoring
        
    -   veya autoencoder (anomaly detection)
        
-   Çıktı:
    
    -   `score` (0-1)
        
    -   `alert` (boolean)
        
-   **Dosya silme / bloklama yok**, sadece uyarı
    

---

## SQLite entegrasyonu

-   `anomaly_scores` tablosu:
    

| Kolon | Tip | Açıklama |
| --- | --- | --- |
| `change_id` | TEXT | İlgili event |
| `file_id` | TEXT | Hangi dosya |
| `peer_id` | TEXT | Hangi peer |
| `score` | REAL | 0-1 arası |
| `alert` | BOOLEAN | threshold geçti mi? |
| `created_at` | INTEGER | timestamp |

-   ML, **delta\_queue** veya **changes** tablosunu besler
    
-   Peer / kullanıcı uyarısı bu tablodan okunur
    

---

## Bilinçli olarak MVP’de yapmadıklarımız

-   Active blocking / quarantine
    
-   Dosya içeriği ML taraması
    
-   Centralized training / cloud model
    

---

## Kararları kilitleyelim

1.  ML = **sadece davranış**
    
2.  Output = **score + alert**, veri silmez
    
3.  Threshold / model MVP’de basit
    
4.  DB’de append-only, audit için saklanır
    

---