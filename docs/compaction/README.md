Role
Maintains LSM health; merges WAL-derived fragments into leveled SSTables and stitches new ANN blocks.

## Scheduler
Runs on indexer nodes only. Priority queue by `(level, age, tombstone-ratio, read-heat)`.

## Compaction Types
| Type   | Trigger             | Effect              |
|-------|--------------------|---------------------|
| Minor | `wal_size > 512 MiB` | `WAL → L0 SST + ΔANN` |
| Level | `ΣLk > 10× target` | `Lk ⨁ Lk→1`          |
| Garbage | `tombstone > 20 %` | rewrite minus deletes |

## Throttling
- Cap total S3 PUTs per tenant (`--max_put_mb=200`).
- Yield to query nodes if NVMe IOPs > 80&nbsp;%.
