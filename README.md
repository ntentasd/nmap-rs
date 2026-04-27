# nmap-rs

A minimal TCP port scanner written in Rust with async I/O.

## Usage

```sh
nmap-rs --address <ADDRESS> --port <PORT> [--tries <TRIES>] [--timeout <MS>]
```

**Options:**

- `-a, --address` — target host (default: `127.0.0.1`)
- `-p, --port` — port to scan (required)
- `-t, --tries` — number of connection attempts (default: `1`)
- `--timeout` — per-attempt timeout in milliseconds (default: `1000`)

## Build

```sh
cargo build --release
```

## Example

```sh
cargo run -- --address 192.168.1.1 --port 80
```
