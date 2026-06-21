use crate::EventObject;
use std::io;
use std::os::fd::RawFd;

/// Platform-agnostic interface for registering file descriptors and waiting for readiness.
///
/// The concrete implementation is [`OsPoller`](crate::OsPoller), which maps to `epoll` on Linux
/// and `kqueue` on macOS.
pub trait Poller {
    /// Create a new OS poller instance.
    fn new() -> io::Result<Self>
    where
        Self: Sized;

    /// Register `event` for **read** readiness.
    ///
    /// The FD and [`EventKind`](crate::EventKind) are packed into the kernel user-data field so
    /// they round-trip when [`wait`](Self::wait) returns.
    fn add(&self, event: EventObject) -> io::Result<()>;

    /// Remove `fd` from the poller.
    fn delete(&self, fd: RawFd) -> io::Result<()>;

    /// Block until at least one registered FD is readable, or the timeout elapses.
    ///
    /// Returns a vector of ready [`EventObject`]s. An empty vector means the call timed out with
    /// no events.
    ///
    /// # Timeout
    ///
    /// | Value | Behavior |
    /// |-------|----------|
    /// | `-1`  | Block indefinitely |
    /// | `0`   | Poll without blocking |
    /// | `> 0` | Timeout in milliseconds |
    fn wait(&mut self, timeout_ms: i32) -> io::Result<Vec<EventObject>>;
}
