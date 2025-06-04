## Pipeline
```
HTTP/gRPC
  └─→ Gateway
        • auth / ratelimit
        • resolves namespace
        • consistent-hash to QE-node
            └─ Query Executor
                 1) parse AST
                 2) pushdown (filters → LSM, KNN → ANN)
                 3) fetch WAL tail
                 4) merge iterators (SST + WAL)
                 5) scorer:
                       • BM25
                       • vector_score (if KNN)
                 6) top-k heap → JSON/Arrow
```

### Concurrency Model
- Each QE uses tokio tasks bound to NUMA cores.
- I/O via `io_uring`; budgeted 4k ops / core.


### API usage
Clients interact through a JSON/HTTP API to insert vectors and issue queries. No SQL interface is provided.

### Planner
A simple planner chooses when to apply lexical filters relative to the ANN search and sets centroid distance limits to balance recall and latency.
