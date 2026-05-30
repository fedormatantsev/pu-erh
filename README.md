# pu-erh

Block-based graph knowledge base.

## Quick start

```bash
cargo build
cargo test
```

Initialize a knowledge base and discover the auto-created root id:

```bash
FILE=/tmp/kb.json
ROOT=$(cargo run -q -p pu-erh-core --example show-root -- "$FILE")
cargo run -q -p pu-erh -- --file "$FILE" create --parent "$ROOT"
cargo run -q -p pu-erh -- --file "$FILE" query "children:$ROOT"
```
