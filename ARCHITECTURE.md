### recallmon in a nutshell

A **first-stage retrieval engine** that keeps *all* durable state in cheap object storage and pulls only the **hot shards** onto local NVMe (and RAM) on-demand.
When the working set is cached it performs like an in-memory system; when it is not, a cold query takes a few ranged reads from S3.

```
┌────────────┐        ┌───────────────────── Cloud ─────────────────────┐
│  Client /  ├─HTTPS─►│  API-gateway & LB   │                            │
│  SDKs      │        │  (routes by region) │                            │
└────────────┘        └┬────────────────────┴──────────────────────────┬─┘
                       ▼                                                ▼
            ╔═════════════════ Query pods (./tpuf) ═══════════════╗     Indexer pods
            ║  • Serve reads & writes (WAL → object storage)      ║       (auto-scaled)
            ║  • Maintain per-namespace SSD/RAM cache            ║  build/merge ANN
            ║  • Perform ANN + BM25 + filtering                  ║  /FTS indexes async
            ║  - strong / eventual-consistency switch            ║
            ╚════════════════════════════════════════════════════╝
                       ▲   NVMe SSD         ▲ object storage (S3/GCS)
                       └──────── cache ─────┘  true source of truth
```

## Key design choices

| Design decision                   | Why it matters                                                                    |
| --------------------------------- | --------------------------------------------------------------------------------- |
| **Object-storage-native LSM WAL** | Infinite, cheap, 11 x durability; no replication of SSDs required.                |
| **Write-through NVMe/RAM cache**  | Warm queries in ≈ 10-20 ms while keeping operating cost low.                      |
| **Centroid-based ANN** | Minimises round-trips on cold reads and supports incremental in-place updates. |
| **Compute/compute separation**    | Query nodes stay lightweight; heavy index builds run on ephemeral “indexer” pods. |
| **Multi-tenant by prefix**        | Millions of namespaces, each independently cached / evicted.                      |


## Components
Detailed documentation for each subsystem lives under the [`docs/`](./docs) directory:

- [Write-Ahead Log](docs/wal/README.md)
- [SSTables](docs/sstable/README.md)
- [ANN blocks](docs/ann-block/README.md)
- [Compaction service](docs/compaction/README.md)
- [Caching tiers](docs/caching/README.md)
- [Query executor](docs/query-executor/README.md)
- [BM25 scoring](docs/bm25/README.md)
- [Consistency model](docs/consistency/README.md)
- [Transactions](docs/transactions/README.md)
