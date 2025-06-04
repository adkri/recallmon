Purpose
Immutable, sorted chunk emitted by the ingest pipeline and merged by compaction. All point and range look-ups hit SSTables first, then the WAL tail.

## Physical Layout
```
| data blocks … | 4-KiB   |
| restart table | 4-KiB   |
| footer        | 128 B   |
```
Data block: 64&nbsp;KiB, Snappy-compressed key+value pairs with prefix compression.
Restart table: `u32` offsets every N=16 keys for `O(log n)` binary search.
Footer: magic, CRC, absolute offset of restart table.

### On-Disk Key
`tenant-id | table-id | primary-key | logical-ts`

## APIs
- `iterator.seek(key)` – binary search restart table; scans block in-memory.
- `meta()` – returns min/max PK, tombstone flag, bloom-filter.

### Bloom Filter
- 10 bits / key → <1&nbsp;% false positives.
- Stored as raw bitset in file trailer.
