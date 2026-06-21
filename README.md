# pollio

A minimal, cross-platform event poller for Linux and macOS. **pollio** wraps native readiness APIs — [epoll](https://man7.org/linux/man-pages/man7/epoll.7.html) on Linux and [kqueue](https://www.freebsd.org/cgi/man.cgi?query=kqueue) on macOS — behind a small, uniform Rust trait.

Use it as a building block for event-driven servers, custom async runtimes, or anywhere you want direct control over the OS poller without pulling in a full I/O framework.

## Features

- **Single trait** — `Poller` abstracts platform differences behind one interface.
- **Native backends** — `epoll` on Linux, `kqueue` on macOS.
- **User data tagging** — attach a lightweight `EventKind` (e.g. server vs. client socket) to each registered FD; it round-trips through the kernel and comes back on wake.
- **Zero runtime dependencies** — only `libc` on supported platforms; no async executor required.
- **Large event batch** — up to 20,000 events per `wait` call.

## Supported platforms

| Platform | Backend |
|----------|---------|
| Linux    | `epoll` |
| macOS    | `kqueue` |

Other targets are not supported. `OsPoller` is exported only on Linux and macOS via `cfg`.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
pollio = "0.1"
```

Or from git:

```toml
pollio = { git = "https://github.com/arrxy/pollio" }
```

## Quick start

```rust
use pollio::{EventObject, OsPoller, Poller};

fn main() -> std::io::Result<()> {
    let mut poller = OsPoller::new()?;

    // Register a listening socket as a "server" FD.
    let listen_fd = /* ... */;
    poller.add(EventObject::server(listen_fd))?;

    loop {
        // Block until at least one FD is readable (-1 = no timeout).
        let ready = poller.wait(-1)?;

        for event in ready {
            match event.kind {
                pollio::EventKind::Server => { /* accept new connections */ }
                pollio::EventKind::Client => { /* read from client */ }
            }
        }
    }
}
```

## API overview

### `Poller` trait

| Method | Description |
|--------|-------------|
| `new()` | Create a new OS poller instance. |
| `add(event)` | Register an FD for **read** readiness. |
| `delete(fd)` | Remove an FD from the poller. |
| `wait(timeout_ms)` | Block until events are ready. Returns a `Vec<EventObject>`. |

**Timeout:** pass `-1` to block indefinitely, `0` to poll without blocking, or a positive value for a timeout in milliseconds.

### `EventObject`

Represents a registered file descriptor and its application-defined role:

```rust
EventObject::server(fd)  // EventKind::Server
EventObject::client(fd)  // EventKind::Client
```

When an FD becomes readable, `wait` returns the corresponding `EventObject` with the same `fd` and `kind`. Internally, `encode` / `decode` pack the FD and kind into the user-data field the kernel returns (`epoll_event.u64` or `kevent.udata`).

### `EventKind`

```rust
pub enum EventKind {
    Server,
    Client,
}
```

Extend or replace this enum in your own fork if you need more roles; the encoding reserves the low 8 bits for the tag and the FD in the upper bits.

## Design notes

- **Read-only today** — registrations use `EPOLLIN` (Linux) and `EVFILT_READ` (macOS). Write readiness and edge-triggered modes are not exposed yet.
- **FD ownership** — pollio does not close registered FDs; you manage their lifecycle.
- **Thread safety** — `OsPoller` is not `Sync`; typical use is a single thread driving the event loop.
- **Error handling** — syscalls surface as `std::io::Error` via `last_os_error()`.

## Project layout

```
src/
├── lib.rs      # Public exports and platform selection
├── poller.rs   # Poller trait
├── event.rs    # EventObject, EventKind, encode/decode
├── epoll.rs    # Linux backend
└── kqueue.rs   # macOS backend
```

## License

Licensed under the MIT License.

## Contributing

Issues and pull requests are welcome at [github.com/arrxy/pollio](https://github.com/arrxy/pollio).
