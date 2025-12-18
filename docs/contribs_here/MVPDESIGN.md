# **Sentinel Edge MVP Tasarım Sayfası**

## Sistem Akışı (High-Level)

```pgsql
[FS Watcher] 
      │
      ▼
[Debounce & Snapshot] 
      │
      ▼
[Delta Engine] ──> [Delta Queue / SQLite] ──┐
      │                                    │
      ▼                                    ▼
  [Peer Transport]                     [ML Anomaly Analysis]
      │                                    │
      ▼                                    ▼
[Trusted Peers] ←─────────────── Alerts & Scores
```

-   FS Watcher → OS-level event tetikler
    
-   Snapshot → immutable, block-level
    
-   Delta Engine → literal / reference delta üretir
    
-   Delta Queue → append-only SQLite, crash safe
    
-   Peer Transport → TLS 1.3 + mTLS, trust kontrolü
    
-   ML → peer davranışı analizi, score + alert
    

---

## SQLite Şeması (Özet)

| Tablo | Amaç |
| --- | --- |
| peers | Peer kimliği, trust, last\_seen |
| files | File metadata, current\_heads, path |
| changes | Event log, delta hash + payload |
| delta\_queue | Gönderilecek değişimler, status |
| file\_snapshots | Snapshot hash, block map, timestamp |
| anomaly\_scores | ML output: score + alert |

-   Append-only, WAL mode
    
-   Disk: dosya verisi, BLOB minimal
    
-   Delta ve snapshot → replayable
    

---

## Peer ve Trust Modeli

-   Peer kimliği = **Ed25519 public key hash**
    
-   Trust = explicit (trusted / revoked)
    
-   TLS 1.3 + mTLS transport
    
-   Revoked peer → izolasyon, data silinmez
    

---

## Event ve Delta Modeli

-   Event immutable, branch-based, overwrite yok
    
-   Event = `change_id`, `file_id`, `parent_change`, `peer_id`, `delta_payload`, `timestamp`
    
-   Conflict → branch oluşur, kullanıcı bilgilendirilir
    
-   Path metadata amaçlı, file\_id eşsiz
    

---

## ML Anomali Tarama

-   Sinyaller: delta frekansı, entropy, branch rate, block pattern
    
-   Output: score + alert (dosya silmez / bloklamaz)
    
-   SQLite `anomaly_scores` tablosuna kaydedilir
    

---

## Kritik Kurallar / MVP Sınırları

-   Dosya overwrite yok, branch üzerinden yönetim
    
-   Otomatik discovery / DHT MVP’de yok
    
-   Dosya içi ML taraması MVP’de yok
    
-   Active blocking / quarantine yok
    
-   Crash & offline recovery SQLite + append-only + delta graph ile sağlanır
    

---

