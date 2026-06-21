use super::{EventObject, Poller};
use libc::{
    EPOLL_CTL_ADD, EPOLL_CTL_DEL, EPOLLIN, close, epoll_create1, epoll_ctl, epoll_event, epoll_wait,
};
use std::io;
use std::os::unix::io::RawFd;
use std::ptr;

const MAX_EVENTS: usize = 20_000;

/// Linux `epoll` backend implementing [`Poller`](crate::Poller).
///
/// Holds up to 20,000 events per [`wait`](Poller::wait) call. The underlying epoll FD is closed
/// on drop.
pub struct OsPoller {
    epoll_fd: RawFd,
    events: Vec<epoll_event>,
}

impl Poller for OsPoller {
    fn new() -> io::Result<Self> {
        let epoll_fd = unsafe { epoll_create1(0) };
        if epoll_fd == -1 {
            return Err(io::Error::last_os_error());
        }
        let events = vec![epoll_event { events: 0, u64: 0 }; MAX_EVENTS];
        Ok(Self { epoll_fd, events })
    }

    fn add(&self, event_object: EventObject) -> io::Result<()> {
        let mut event = epoll_event {
            events: EPOLLIN as u32,
            u64: event_object.encode() as u64,
        };
        let result =
            unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, event_object.fd, &mut event) };
        if result == -1 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }

    fn delete(&self, fd: RawFd) -> io::Result<()> {
        let result = unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_DEL, fd, ptr::null_mut()) };
        if result == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    fn wait(&mut self, timeout_ms: i32) -> io::Result<Vec<EventObject>> {
        let nevents = unsafe {
            epoll_wait(
                self.epoll_fd,
                self.events.as_mut_ptr(),
                self.events.len() as i32,
                timeout_ms,
            )
        };

        if nevents == -1 {
            return Err(io::Error::last_os_error());
        }
        let mut ready = Vec::with_capacity(nevents as usize);
        for i in 0..nevents as usize {
            let data = self.events[i].u64 as usize;
            ready.push(EventObject::decode(data));
        }
        Ok(ready)
    }
}

impl Drop for OsPoller {
    fn drop(&mut self) {
        unsafe {
            close(self.epoll_fd);
        }
    }
}
