mod event;
mod poller;

pub use event::{EventKind, EventObject};
pub use poller::Poller;

#[cfg(target_os = "linux")]
mod epoll;

#[cfg(target_os = "macos")]
mod kqueue;

#[cfg(target_os = "linux")]
pub use epoll::OsPoller;

#[cfg(target_os = "macos")]
pub use kqueue::OsPoller;