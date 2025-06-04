## Parameters
- `k1 = 1.2`
- `b = 0.75`
- Field-level boosts configurable in `schema.yml`.

### Indexing
Classic inverted index (Roaring + positions) lives in the same SSTable:
- term-dict → postings (delta-varint)
- positions zipped with SIMD-BP128

### Query-time
- Parse, de-stem, stop-filter.
- Intersect postings (AND) / union (OR).
- Accumulate BM25, spill into top-K heap shared with ANN results.
