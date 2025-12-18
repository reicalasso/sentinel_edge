## SQLITE — VERİ YÖNETİMİ

### Amaç

-   Peer metadata, file state, change log, delta queue, ML skorlarını tek merkezden yönetmek
    
-   ACID ile güvenilir
    
-   Append-only + immutable yaklaşımıyla conflict ve crash güvenliği
    

---

## Temel tablolar ve işlevleri

### `peers` — peer kimliği ve trust

| Kolon | Tip | Açıklama |
| --- | --- | --- |
| `peer_id` | TEXT | Ed25519 public key hash |
| `public_key` | BLOB | Peer’in public key’i |
| `trust_state` | TEXT | trusted / revoked |
| `first_seen` | INTEGER | timestamp |
| `last_seen` | INTEGER | timestamp |

---

### `files` — dosya meta bilgisi

| Kolon | Tip | Açıklama |
| --- | --- | --- |
| `file_id` | TEXT | UUID / hash tabanlı |
| `current_heads` | TEXT | JSON array / change\_id listesi |
| `path` | TEXT | Kullanıcı yolu, sadece metadata |
| `created_at` | INTEGER | timestamp |
| `deleted` | BOOLEAN | dosya silinmiş mi? |

---

### `changes` — event log

| Kolon | Tip | Açıklama |
| --- | --- | --- |
| `change_id` | TEXT | Delta hash veya UUID |
| `file_id` | TEXT | Hangi dosya |
| `parent_change` | TEXT | Önceki change\_id |
| `peer_id` | TEXT | Kim üretti |
| `delta_hash` | BLOB | Delta içeriği hash’i |
| `delta_data` | BLOB | Compressed delta (opsiyonel) |
| `created_at` | INTEGER | timestamp |

---

### `delta_queue` — peer’a gönderilecek değişimler

| Kolon | Tip | Açıklama |
| --- | --- | --- |
| `change_id` | TEXT | Queue’daki event ID |
| `file_id` | TEXT |  |
| `peer_id` | TEXT | Hedef peer |
| `status` | TEXT | pending / sent / applied |
| `created_at` | INTEGER |  |

---

### `file_snapshots` — snapshot + block map

| Kolon | Tip | Açıklama |
| --- | --- | --- |
| `file_id` | TEXT |  |
| `snapshot_hash` | BLOB | Rolling hash veya Merkle root |
| `block_map` | BLOB | Block-level hash listesi |
| `timestamp` | INTEGER | snapshot zamanı |

---

### SQLite kullanımıyla ilgili kritik kararlar

1.  **Append-only tablo mantığı** → rollback / audit kolay
    
2.  **BLOB’ları disk dışı tutma** → büyük dosya blokları ayrı dosyada
    
3.  **WAL modunda aç** → crash-safe
    
4.  **Transaction sınırı** → kısa ve hızlı
    
5.  **Indexleme** → `file_id`, `change_id` ve `peer_id` mutlaka
    

---

### Bilinçli olarak yapmadıklarımız

-   Multi-master concurrency complexity (MVP’de peer queue yeter)
    
-   Komple merge engine (branch’ler offline kalır)
    
-   Heavy analytics (ML minimum veri ile çalışacak)
    

---

### Kararları Kilitleyelim

-   SQLite = state + log + queue
    
-   Dosya verisi = disk
    
-   Delta = compressed BLOB
    
-   ML ve transport queue doğrudan buradan beslenir
    

---
