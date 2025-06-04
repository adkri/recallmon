Purpose
Houses a centroid-partitioned HNSW index for ±1 million vectors.

## Layout (.ann)
```
HEADER          –  1 KiB (dims, metric, M, ef)
CENTROIDS       –  C * 16 KiB  (float32[dim] + child-count)
VEC-HEAP        –  contiguous, quantised (f16 or pq-8-bit)
NEIGHBOR LISTS  –  var-int adjacency
FOOTER (CRC32)
```

## Build Pipeline
1. K-means++ (mini-batch) → centroids (`C=√N`).
2. Assign vectors → shards, quantise.
3. Build per-shard HNSW (`M=16`, `ef_con=200`).
4. Spill to S3 with `X-Amz-Tagging=ann`.

## Query Flow
1. load centroid slab (vector) – 1 GET
2. select top-K shards (dot/cosine) – CPU
3. bulk fetch shard HNSWs in parallel – ≤K GETs
4. intra-shard HNSW search (`ef_runtime`) – CPU

### Hot-Patch Updates
Small inserts go to a mutable RAM-graph flushed into the next compaction round.
