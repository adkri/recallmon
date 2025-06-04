Read-Your-Own-Write (RYOW) for single client.

Linearizable WAL appends: `(If-None-Match)` + cross-AZ submit + quorum-etag check.

Queries default to bounded-staleness ≤1&nbsp;s; caller may request `staleness=0` (forces list-prefix poll).
