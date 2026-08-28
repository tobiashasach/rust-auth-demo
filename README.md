# Demo Auth

## RSA Keygen

With openssl installed use:

```
openssl genpkey -algorithm RSA -out ./keys/private_key.pem -pkeyopt rsa_keygen_bits:2048

openssl pkey -in ./keys/private_key.pem -pubout -out ./keys/public_key.pem
```

This generates a private and public keypair with keylen 2048 in ./keys

---

<strong>This repository is for learning and demo purposes only</strong>