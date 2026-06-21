use std::os::fd::RawFd;

/// Application-defined role for a registered file descriptor.
///
/// When an FD becomes readable, [`EventObject::kind`] is restored from the value supplied at
/// registration time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    /// A listening / server socket.
    Server,
    /// A connected client socket.
    Client,
}

/// A registered file descriptor and its application-defined role.
///
/// Construct with [`EventObject::server`] or [`EventObject::client`], then pass to
/// [`Poller::add`](crate::Poller::add). When the FD becomes readable,
/// [`Poller::wait`](crate::Poller::wait) returns the same `fd` and `kind`.
#[derive(Debug, Clone, Copy)]
pub struct EventObject {
    /// The raw file descriptor being polled.
    pub fd: RawFd,
    /// The role assigned when the FD was registered.
    pub kind: EventKind,
}

impl EventObject {
    /// Register `fd` as a server (listening) socket.
    pub fn server(fd: RawFd) -> Self {
        Self {
            fd,
            kind: EventKind::Server,
        }
    }

    /// Register `fd` as a client (connected) socket.
    pub fn client(fd: RawFd) -> Self {
        Self {
            fd,
            kind: EventKind::Client,
        }
    }

    /// Pack `fd` and `kind` into the user-data word stored by the kernel.
    ///
    /// The low 8 bits hold the kind tag; the FD occupies the upper bits.
    pub fn encode(self) -> usize {
        let tag = match self.kind {
            EventKind::Server => 1usize,
            EventKind::Client => 2usize,
        };
        ((self.fd as usize) << 8) | tag
    }

    /// Restore an [`EventObject`] from a kernel user-data word.
    ///
    /// # Panics
    ///
    /// Panics if `data` contains an unknown kind tag.
    pub fn decode(data: usize) -> Self {
        let tag = data & 0xff;
        let fd = (data >> 8) as RawFd;

        let kind = match tag {
            1 => EventKind::Server,
            2 => EventKind::Client,
            _ => panic!("unknown event kind"),
        };

        Self { fd, kind }
    }
}
