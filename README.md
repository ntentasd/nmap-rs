# nmap-rs

A minimal TCP port scanner written in Rust with async I/O.

## Usage

```sh
nmap-rs --address <ADDRESS> --ports <PORTS> [OPTIONS]
```

**Options:**

- `-a, --address` — target host (default: `127.0.0.1`)
- `-p, --ports` — ports to scan: single (`80`), range (`22-100`), or list (`22,80,443`) (required)
- `-t, --tries` — number of connection attempts per port (default: `1`)
- `-j, --concurrency` — max concurrent scans (default: `5`)
- `--timeout` — per-attempt timeout in milliseconds (default: `1000`)
- `--all` — show all ports including closed and timed out (default: open only)

## Build

```sh
cargo build --release
```

## Example

```sh
cargo run -- --address 192.168.1.1 --ports 22-443
cargo run -- --address 192.168.1.1 --ports 22,80,443 --all
```
