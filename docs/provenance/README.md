# Cryptographic provenance (IP doc Stage 0.2)

OpenTimestamps receipts anchoring Laverna's git HEAD hashes into the Bitcoin
blockchain. These corroborate *existence + integrity + signed authorship* of a
given milestone; they are belt-and-suspenders alongside US copyright
registration (which unlocks statutory damages) — NOT a substitute for it.

Verify:  ots verify <file>.ots
Upgrade: ots upgrade <file>.ots   (once the stamp is in a confirmed block)

NOTE: the `.ots` receipt is created pending Bitcoin block confirmation
(~1-2h after stamping). Until then `ots upgrade` reports
"Timestamp not complete" and `ots verify` shows "Pending confirmation".
This is normal — re-run `ots upgrade` later, then `ots verify` will
print the block height. The original digest is stored as `HEAD-431bbea`
(basename MUST match the `.ots` file, minus extension) so verify works.

| receipt | git HEAD | meaning |
|----------|----------|---------|
| HEAD-431bbea.ots | 431bbea0a1541142e2138d04d7ff38cd7d084b5a | Apache-2.0 relicense milestone |

Signing key: `laverna-signing-pubkey.asc` (Ed25519, nutypebuddha).
All commits from 431bbea onward are GPG-signed (git log shows `G`).
