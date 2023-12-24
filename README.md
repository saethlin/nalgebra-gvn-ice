ICE with
```
RUSTFLAGS="-Zmir-opt-level=0 -Zmir-enable-passes=+GVN -Zinline-mir -Zcross-crate-inline-threshold=always -Zinline-mir-hint-threshold=10000" cargo +nightly build
```
