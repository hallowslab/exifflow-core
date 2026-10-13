# exifflow-core

Shared, transport-agnostic upload plumbing for ExifFlow transfer front-ends.
Extracted from `rftps` so the FTP server and the HTTP upload server (`recv`)
publish identical events and write the same durable replication records.

## Contents

| Module | Purpose |
| ------ | ------- |
| `event` | `EventBus` + `FtpEvent`, the in-process activity stream |
| `record` | `RecordStore` (SQLite, WAL) + `resolve_upload_path` |
| `utils` | local IP resolution, upload-root validation, random strings, cert checks |

The `record` schema is the contract with `replicator-rs`; keep it in sync.

## Usage

```toml
[dependencies]
exifflow-core = { git = "https://github.com/hallowslab/exifflow-core", tag = "v0.0.1" }
```

The ExifFlow workspace overrides this with a local path via `[patch]`, so local
development builds offline.

## Build

```bash
cargo build --release
cargo test
```
