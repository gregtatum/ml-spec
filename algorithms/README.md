# Algorithms

Algorithm implementations and specs that back up machine learning fundamentals.

## Run

```bash
cargo test -p algorithms
```

## Levenshtein Distance

This project includes three Levenshtein distance implementations in
`algorithms/src/distance.rs`:

- `levenstein_distance_ref`: a reference dynamic programming implementation
  that materializes a full (m+1) x (n+1) table for clarity.
- `levenstein_distance_opt`: an optimized variant that uses a single DP row and
  early-exit fast paths, with O(min(m, n)) space.
- `levenstein_distance_byte_opt`: a byte-based optimized version that operates
  on UTF-8 bytes (faster and lower allocation, but semantics differ from the
  Unicode scalar approach for non-ASCII input).

All three share the same ASCII test cases, and the byte variant is expected to
match those results.
