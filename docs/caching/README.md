## Tiers
| Layer | Medium      | Hit µ-lat   | Eviction      |
|------|-------------|-------------|---------------|
| L0   | heap (arena) | 0.1–2 µs   | CLOCK-pro     |
| L1   | NVMe SSD (ZNS) | 25–150 µs | LFU-segmented |
| L2   | S3 / GCS    | 100–400 ms | N/A           |

### Admission
- Tiny WAL pages always L0.
- SST/ANN blocks: admit on 2nd hit (TinyLFU).

### Metadata
`cache_index` TinyHash keyed by S3 ETag to detect stale objects.
