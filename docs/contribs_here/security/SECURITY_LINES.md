## GÜVENLİK SINIRLARI

### Amaç

-   Peer’lar arası bağlantı güvenli olsun
    
-   Data integrity sağlansın
    
-   Replay ve tampering önlensin
    
-   Trust yönetimi kontrol altında olsun
    

---

## Transport Security

1.  **TLS 1.3**
    
    -   Minimum requirement
        
    -   Forward secrecy
        
    -   AEAD cipher (GCM veya ChaCha20-Poly1305)
        
2.  **Mutual TLS (mTLS)**
    
    -   Peer kimliği = sertifika public key
        
    -   Handshake sırasında doğrulama
        
3.  **Peer Revocation**
    
    -   `revoked` peer = bağlantı engellenir
        
    -   Gönderilen değişimler işlenmez
        
    -   History silinmez (audit purpose)
        

---

## Data Integrity

-   Her delta **hash’li** ve immutable
    
-   Queue ve change log append-only
    
-   Peer ID + change hash → **imzalanabilir event**
    
-   Transport katmanı sadece taşıyor, integrity protokol katmanı garanti ediyor
    

---

## Replay ve tampering önleme

-   `change_id` monotonik veya UUID tabanlı
    
-   Peer log ve DB append-only
    
-   Duplicate change = ignore
    
-   TLS sequence / MAC → network layer
    

---

## Crash ve offline senaryoları

-   SQLite WAL modu + append-only → crash recovery
    
-   Peer offline → event queue’da bekler
    
-   Peer geri geldiğinde replayable event graph uygulanır
    
-   Overwrite yok, branch korunur
    

---

## Özet kritik kararlar

-   Peer kimliği = **kripto public key**
    
-   Trust = **explicit, geri alınabilir**
    
-   Transport = **TLS 1.3 + mTLS**
    
-   Event = **hash + immutable**
    
-   Replay / duplicate = **DB ve delta graph**
    
-   Conflict = **branch, overwrite yok**
    

---

### Yapmadıklarımız

-   Otomatik trust (invite only)
    
-   Global DHT / discovery (MVP)
    
-   Zero-trust tüm peer’lar (sadece trusted peer)
    
-   File encryption at rest (opsiyonel, sonra)
    

---

### Kararları kilitleyelim

Artık **her peer bağlantısı ve veri transferi güvenli**,  
ve **yapısal olarak safe**.

---