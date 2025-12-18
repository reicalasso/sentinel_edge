## PEER MODELİ — KİMLİK & GÜVEN

### Amaç

-   Peer’lar **değişse bile** kimlik değişmesin
    
-   IP, hostname, NAT **önemsiz** olsun
    
-   Güven **varsayım değil**, **bilinçli karar** olsun
    

---

## Peer kimliği

**Peer = kriptografik kimlik**

-   Algoritma: **Ed25519**
    
-   `peer_id` = public key’in hash’i (örn. BLAKE3)
    
-   Private key **asla paylaşılmaz**
    
-   Peer adı sadece **etiket** (güven kaynağı değil)
    

> IP adresi = konum  
> Peer ID = kimlik

---

## Trust modeli (explicit, geri alınabilir)

### İlk bağlantı (MVP)

-   **Out-of-band trust**
    
    -   fingerprint karşılaştırma
        
    -   QR / text token
        
-   Kullanıcı **bilerek** trust verir
    

### Trust seviyeleri

Basit ama yeterli:

-   `trusted`
    
-   `revoked`
    

Başlangıçta başka seviye eklemeyeceğiz. Karmaşıklık öldürür.

---

## Bağlantı güvenliği (pratik ve sağlam)

-   **TLS 1.3**
    
-   **Mutual TLS**
    
-   Sertifika = peer public key
    
-   Handshake’te:
    
    -   peer\_id doğrulanır
        
    -   trust table’da mı bakılır
        

> Şifreleme transport’ta  
> Kimlik doğrulama protokolde

---

## Trust kırılırsa ne olur?

Bu net olmalı.

-   Peer **bağlanamaz**
    
-   Yeni event **alınmaz**
    
-   Eski veriler **silinmez**
    
-   Audit log tutulur
    

Yani:

> Trust kaldırmak = izolasyon  
> Silme değil

---

## SQLite’da tutulacak minimum tablolar

(Detaya boğmuyorum, sadece fikrim)

-   `peers`
    
    -   `peer_id`
        
    -   `public_key`
        
    -   `trust_state`
        
    -   `first_seen`
        
    -   `last_seen`
        
-   `connections`
    
    -   `peer_id`
        
    -   `last_ip`
        
    -   `last_handshake`
        
    -   `tls_fingerprint`
        

---

## Bilinçli olarak YAPMADIKLARIMIZ

Bugün için özellikle yok:

-   ❌ otomatik peer discovery
    
-   ❌ public DHT
    
-   ❌ anonymous peer
    
-   ❌ “herkesle sync”
    

Bunlar **sonra**, belki.

## Kararları kilitleyelim

Şu kararlar artık değişmiyor:

-   Peer = anahtar
    
-   Trust = explicit
    
-   TLS = mTLS
    
-   Revocation = izolasyon
