Purpose
The Write-Ahead Log (WAL) is the source-of-truth event stream. Every mutation is appended here before it becomes visible anywhere else.

## File Layout
| Byte Range | Field        | Notes                             |
|-----------:|--------------|-----------------------------------|
| 0-7        | u64 magic    | `0x57414C3032` ("WAL02")          |
| 8-15       | u64 txn-id   | Monotonic, globally unique        |
| 16-23      | u64 timestamp-µs | Physical clock               |
| 24-31      | u64 checksum | xxHash64 of payload               |
| 32-…       | CBOR payload | Insert / Delete / Upsert JSON doc |

One file per WAL segment. Each segment is sized to 256&nbsp;MiB (configurable) and named `0000000000000000.wal`, `…0001.wal`, …

## Operations
| API                       | Action                                                             |
|---------------------------|--------------------------------------------------------------------|
| `append(bytes entry)`     | Atomically PUTs new object to S3: `wal/$seg/$offset` with `If-None-Match:"*"` |
| `seal(seg#)`              | Writes zero-byte `wal/$seg/⟂sealed` object.                       |

## Crash-Recovery
1. List objects under current segment.
2. Replay in offset order, verifying checksum.
3. Stop at first corrupt / missing gap.

## Metrics
- `wal_put_seconds` (histogram)
- `wal_replication_lag` (bytes behind compactor)

## Tuning Knobs
- `SEG_SIZE` – 64&nbsp;MiB – 1&nbsp;GiB
- `FLUSH_INTERVAL_MS` – group-commit window
