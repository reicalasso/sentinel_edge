## FS EVENT → DELTA PIPELINE

### Amaç

-   Dosya değişimlerini **gerçek delta’ya** çevirmek
    
-   Partial write / temp file / editor-trick patlamasını engellemek
    
-   Event-sourced model ile uyumlu hale getirmek
    

---

## Adım adım akış

1.  **FS Watcher**
    
    -   OS-level event alınır
        
    -   Not: Bu **hata yapmaya çok elverişli**
        
        -   Duplicate event
            
        -   Partial write
            
        -   Rename / move / delete
            
    -   OS farkları var: Linux (inotify), Windows (ReadDirectoryChangesW), macOS (FSEvents)
        
2.  **Debounce / coalesce**
    
    -   50–200 ms bekle
        
    -   Aynı dosyada art arda gelen eventleri tekle
        
    -   Ama çok uzun bekleme → latency
        
    -   Çok kısa bekleme → partial write riskli
        
3.  **Snapshot**
    
    -   Event geldi → snapshot al
        
    -   Dosya ID ile eşle
        
    -   Hash block bazlı (rolling hash önerilir)
        
    -   Bu snapshot **immutable** olmalı
        
4.  **Diff / Delta Engine**
    
    -   Önceki snapshot ile karşılaştır
        
    -   Block-level delta üret
        
    -   Literal vs reference ayrımı yapılır
        
    -   Delta hash üretilir → change\_id oluşturulur
        
5.  **Queue / Log**
    
    -   SQLite’a append-only olarak ekle
        
    -   Peer gönderimi için queue’da beklet
        
    -   ML anomali analizine buradan besle
        

---

## Önemli noktalar / tuzaklar

| Tuzak | Çözüm |
| --- | --- |
| Partial write | Snapshot + debounce |
| Editor rename trick | Dosya ID ile takip |
| Temp file | Ignore patterns / whitelist |
| Large file | Block-level delta, streaming |
| Duplicate event | Deduplicate hash veya delta |
| Conflict | Event graph, branch, overwrite yok |

---

## SQLite şeması (pipeline kısmı)

-   `delta_queue`
    
    -   `change_id`
        
    -   `file_id`
        
    -   `peer_id`
        
    -   `delta_data` (compressed)
        
    -   `status` (pending, sent, applied)
        
    -   `created_at`
        
-   `file_snapshots`
    
    -   `file_id`
        
    -   `snapshot_hash`
        
    -   `timestamp`
        
    -   `block_map`
        

---

## Kararları kilitleyelim

-   FS watcher = sadece tetikleyici, **gerçek delta buradan üretilir**
    
-   Partial write = snapshot ile güvence altına alınır
    
-   Delta = immutable, hash’li, block bazlı
    
-   Queue = SQLite append-only, ML ve peer transport için hazır
    

---