## Scope
Single-key idempotent puts & deletes (exactly what we write to the WAL). Multi-row / multi-table ACID is out-of-scope; use app-side sagas.

## Call Flow
```
BEGIN
PUT /txn/prepare  {batch…}   -> 202 + txid
… client retries allowed …
PUT /txn/commit?txid=123     -> 200 OK
```
prepare stage buffered in DynamoDB sticky to tenant.
commit stage writes atomically into next WAL segment and waits for `⟂sealed` ack.
If no commit within TTL (30 s) → automatic rollback.

## Guarantees
- Exactly-once append.
- RYOW for same txid.
- Detected-duplicate idempotency via txn-table.
