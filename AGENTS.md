# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust sources for the mio-backed asyncio event loop. Entry point is `lib.rs`; network primitives live in `tcp.rs`, `udp.rs`, and `sock.rs`, while scheduling/timekeeping code sits in `event_loop.rs` and `time.rs`.
- `rloop/`: Python shim packaged via PyO3. Core policy and loop wrappers are in `loop.py`; transport helpers live under `transports.py` and `server.py`.
- `tests/`: Pytest suite exercising Python-facing APIs. Rust-only tests reside inline within the Rust modules.
- `benchmarks/`: Criterion benchmarks for profiling hot Rust paths.

## Build, Test, and Development Commands
- `make build-dev`: Syncs the uv environment and performs a local `maturin develop` build of the extension module.
- `make format`: Auto-formats Python (ruff) and Rust (`cargo fmt`).
- `make lint`: Runs `ruff` plus `cargo clippy` with repository-specific allowlists to enforce style without blocking complex async code.
- `make test`: Executes the pytest suite with verbose output.
- `cargo test -p rloop`: Useful when iterating on Rust unit tests embedded in `src/`.

## Coding Style & Naming Conventions
- Python: Ruff enforces 120-character lines, single quotes for strings, and PEP 8 naming conventions (enforced by pep8-naming; e.g., `EventLoopPolicy`, `RLoopError`). Prefer async/await-friendly helpers and avoid implicit event-loop globals.
- Rust: Follow `cargo fmt` defaults (4-space indentation). Module names use snake_case; public types favor CamelCase (`EventLoop`, `TcpStreamState`). Use explicit `use` lists to keep FFI boundaries clear.
- Keep Rust↔Python boundaries in `py.rs` thin; place heavy logic in pure Rust modules.

## Testing Guidelines
- Pytest is the source of truth; mimic existing filenames like `test_loop.py` and use `pytest.mark.asyncio` for coroutine tests.
- Target meaningful coverage of transport edge cases and timer ordering; regression tests should reproduce previously failing asyncio scenarios.
- Run `make test` before committing; add Criterion benchmarks only when a performance regression is suspected.

## Commit & Pull Request Guidelines
- Commit subjects follow the short, imperative style seen in history (“Add UDP support”). Use focused commits grouping related changes across Rust and Python layers.
- PRs should summarize behavior changes, list new commands or configuration toggles, and note any compatibility caveats (Unix-only features, Python version assumptions). Link issues when available and attach benchmark or test output for performance-sensitive work.

## Security & Configuration Notes
- No Windows support; guard OS-specific code with `cfg(unix)`.
- Treat external socket input as untrusted—validate buffer lengths before bridging into Python objects.
