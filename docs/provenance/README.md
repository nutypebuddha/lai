# Cryptographic provenance (IP doc Stage 0.2)

OpenTimestamps receipts anchoring Laverna's git HEAD hashes into the Bitcoin
blockchain. These corroborate *existence + integrity + signed authorship* of a
given milestone; they are belt-and-suspenders alongside US copyright
registration (which unlocks statutory damages) — NOT a substitute for it.

Verify:  ots verify <file>.ots
Upgrade: ots upgrade <file>.ots   (once the stamp is in a confirmed block)

| receipt | git HEAD | meaning |
|----------|----------|---------|
| HEAD-431bbea.ots | 431bbea0a1541142e2138d04d7ff38cd7d084b5a | Apache-2.0 relicense milestone |
