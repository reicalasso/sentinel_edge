#  **Sentinel Edge v0.1 – Whitepaper (Tek Sayfa)**

## Problem Tanımı

-   Mevcut dosya senkronizasyonu çözümleri çoğunlukla merkezi, gecikmeli veya overwrite riski taşıyor.
    
-   Peer’lar offline/online geçişlerinde veri kaybı veya sessiz conflict yaşanıyor.
    
-   Dosya içeriği taransa da davranış bazlı anomaliler gözlenmiyor.
    
-   Hedef: **güvenli, P2P, gerçek zamanlı değişim senkronizasyonu + davranış analizi.**
    

---

## Peer Modeli

-   Peer kimliği = **Ed25519 public key hash**
    
-   Trust = **explicit, geri alınabilir** (trusted / revoked)
    
-   Transport = **TLS 1.3 + mutual TLS**
    
-   Revoked peer = bağlantı engellenir, geçmiş veriler silinmez.
    

---

## Değişim Modeli

-   Dosya değil, **immutable change event** senkronize edilir.
    
-   Event yapısı: `change_id`, `file_id`, `parent_change`, `peer_id`, `delta_payload`, `timestamp`
    
-   Conflict → branch, **overwrite yok**
    
-   Path metadata amaçlı, dosya ID eşsiz
    

---

## FS → Delta Pipeline

1.  FS event tetiklenir
    
2.  Debounce / coalesce (50–200ms)
    
3.  Snapshot alınır (block-level hash)
    
4.  Diff / delta üretilir
    
5.  SQLite append-only queue ve change log’a eklenir
    

-   Partial write, rename, delete gibi edge case’ler kontrol altında.
    

---

## SQLite Veri Modeli

| Tablo | Amaç |
| --- | --- |
| `peers` | Peer kimliği, trust, last\_seen |
| `files` | File metadata, current\_heads, path |
| `changes` | Event log, delta hash + payload |
| `delta_queue` | Gönderilecek değişimler, status |
| `file_snapshots` | Snapshot hash, block map, timestamp |
| `anomaly_scores` | ML output: score + alert |

-   SQLite append-only, WAL modu, disk güvenli.
    
-   Dosya verisi disk üzerinde, delta ve snapshot BLOB’ları minimal.
    

---

## Güvenlik

-   Transport = **TLS 1.3 + mTLS**
    
-   Event = **hash + immutable**
    
-   Replay / duplicate kontrol = DB + delta graph
    
-   Crash recovery = append-only + WAL
    
-   Peer revocation = izolasyon, veri silinmez
    

---

## ML Tabanlı Anomali Tarama

-   Hedef: **dosya içeriği değil, peer davranışı**
    
-   Sinyaller: delta frekansı, block pattern, entropy, branch rate
    
-   Output = score + alert, **dosya silmez / bloklamaz**
    
-   SQLite `anomaly_scores` tablosuna kaydedilir
    
-   MVP basit threshold veya autoencoder ile çalışır
    

---

## Sentinel Edge v0.1 Kilit Kararlar

-   **Immutable event-sourced model** → veri kaybı yok
    
-   **Peer trust + crypto identity** → güvenli ve izlenebilir
    
-   **Append-only SQLite + delta queue** → crash safe + audit
    
-   **ML davranış analizi** → anomali uyarısı, müdahaleye açık
    

---

💡 **Not:** Otomatik discovery, global DHT, active blocking ve dosya içi ML taraması MVP’de yok; ileri sürümler için planlı.

---