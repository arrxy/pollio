//! A minimal, cross-platform event poller for Linux and macOS.
//!
//! **pollio** wraps native readiness APIs — [epoll](https://man7.org/linux/man-pages/man7/epoll.7.html)
//! on Linux and [kqueue](https://www.freebsd.org/cgi/man.cgi?query=kqueue) on macOS — behind a small,
//! uniform [`Poller`] trait.
//!
//! Use it as a building block for event-driven servers, custom async runtimes, or anywhere you want
//! direct control over the OS poller without pulling in a full I/O framework.
//!
//! # Supported platforms
//!
//! | Platform | Backend | Type alias |
//! |----------|---------|------------|
//! | Linux    | `epoll` | [`OsPoller`] |
//! | macOS    | `kqueue`| [`OsPoller`] |
//!
//! Other targets are not supported. [`OsPoller`] is exported only on Linux and macOS.
//!
//! # Quick start
//!
//! ```no_run
//! use pollio::{EventObject, OsPoller, Poller};
//!
//! fn main() -> std::io::Result<()> {
//!     let mut poller = OsPoller::new()?;
//!
//!     // Register a listening socket as a "server" FD.
//!     let listen_fd = 0; // replace with your listening socket
//!     poller.add(EventObject::server(listen_fd))?;
//!
//!     loop {
//!         // Block until at least one FD is readable (-1 = no timeout).
//!         let ready = poller.wait(-1)?;
//!
//!         for event in ready {
//!             match event.kind {
//!                 pollio::EventKind::Server => { /* accept new connections */ }
//!                 pollio::EventKind::Client => { /* read from client */ }
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! # Design notes
//!
//! - **Read-only today** — registrations use `EPOLLIN` (Linux) and `EVFILT_READ` (macOS). Write
//!   readiness and edge-triggered modes are not exposed yet.
//! - **FD ownership** — pollio does not close registered FDs; you manage their lifecycle.
//! - **Thread safety** — [`OsPoller`] is not [`Sync`]; typical use is a single thread driving the
//!   event loop.
//! - **Error handling** — syscalls surface as [`std::io::Error`] via `last_os_error()`.

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod event;
mod poller;

pub use event::{EventKind, EventObject};
pub use poller::Poller;

#[cfg(target_os = "linux")]
mod epoll;

#[cfg(target_os = "macos")]
mod kqueue;

#[cfg(target_os = "linux")]
#[cfg_attr(docsrs, doc(cfg(target_os = "linux")))]
pub use epoll::OsPoller;

#[cfg(target_os = "macos")]
#[cfg_attr(docsrs, doc(cfg(target_os = "macos")))]
pub use kqueue::OsPoller;
