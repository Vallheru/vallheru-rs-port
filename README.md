# Vallheru

## Development

### Prepare environment

Prerequisites:

- [Bun](https://bun.sh/docs/installation) installed
- [Rust and cargo](https://www.rust-lang.org/learn/get-started) installed
- [Node.js(v20.0.0+) installed](https://nodejs.org/en/download) installed
- `pkg-config` package installed

Visual studio codes recommended extensions

- Tailwind intellisense
- Rust analyzer
- CodeLLDB

```shell
make install-deps
```

Project generated with [leptos-starter-csr app](https://github.com/leptos-community/start-csr)

## Contributions

```rs
cd frontend && trunk watch
cd backend && cargo watch -q -c -w src/ -x "run"
cd backend && cargo watch -q -c -w examples/ -x "run --example quick_dev"
```
