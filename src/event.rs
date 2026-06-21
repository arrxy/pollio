use std::os::fd::RawFd;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    Server,
    Client,
}

#[derive(Debug, Clone, Copy)]
pub struct EventObject {
    pub fd: RawFd,
    pub kind: EventKind,
}

impl EventObject {
    pub fn server(fd: RawFd) -> Self {
        Self {
            fd,
            kind: EventKind::Server,
        }
    }

    pub fn client(fd: RawFd) -> Self {
        Self {
            fd,
            kind: EventKind::Client,
        }
    }

    pub fn encode(self) -> usize {
        let tag = match self.kind {
            EventKind::Server => 1usize,
            EventKind::Client => 2usize,
        };
        ((self.fd as usize) << 8) | tag
    }

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