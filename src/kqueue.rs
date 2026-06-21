use crate::{
    poller::Poller,
    EventObject
};
use libc::{EV_ADD, EV_DELETE, EV_ENABLE, EVFILT_READ, close, kevent, kqueue, timespec};
use std::{io, os::fd::RawFd, ptr};

const MAX_EVENTS: usize = 20_000;

/// macOS `kqueue` backend implementing [`Poller`](crate::Poller).
///
/// Holds up to 20,000 events per [`wait`](Poller::wait) call. The underlying kqueue FD is closed
/// on drop.
pub struct OsPoller {
    kqueue_fd: RawFd,
    events: Vec<kevent>,
}

impl Poller for OsPoller {
    fn new() -> io::Result<Self> {
        let kqueue_fd = unsafe { kqueue() };
        if kqueue_fd == -1 {
            return Err(io::Error::last_os_error());
        }
        let events = vec![unsafe { std::mem::zeroed::<kevent>() }; MAX_EVENTS];
        Ok(Self { kqueue_fd, events })
    }

    fn add(&self, event: EventObject) -> Result<(), std::io::Error> {
        let mut change = libc::kevent {
            ident: event.fd as libc::uintptr_t,
            filter: EVFILT_READ,
            flags: EV_ADD | EV_ENABLE,
            fflags: 0,
            data: 0,
            udata: event.encode() as *mut libc::c_void,
        };
        let result = unsafe {
            kevent(
                self.kqueue_fd,
                &mut change,
                1,
                ptr::null_mut(),
                0,
                ptr::null(),
            )
        };
        if result == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    fn delete(&self, fd: RawFd) -> Result<(), std::io::Error> {
        let mut change = libc::kevent {
            ident: fd as libc::uintptr_t,
            filter: EVFILT_READ,
            flags: EV_DELETE,
            fflags: 0,
            data: 0,
            udata: ptr::null_mut(),
        };
        let result = unsafe {
            kevent(
                self.kqueue_fd,
                &mut change,
                1,
                ptr::null_mut(),
                0,
                ptr::null(),
            )
        };
        if result == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    fn wait(&mut self, timeout_ms: i32) -> io::Result<Vec<EventObject>> {
        let timeout_storage;

        let timeout_ptr: *const timespec = if timeout_ms < 0 {
            std::ptr::null()
        } else {
            timeout_storage = timespec {
                tv_sec: (timeout_ms / 1000) as libc::time_t,
                tv_nsec: ((timeout_ms % 1000) * 1_000_000) as libc::c_long,
            };

            &timeout_storage as *const libc::timespec
        };

        let nevents = unsafe {
            libc::kevent(
                self.kqueue_fd,
                std::ptr::null(),
                0,
                self.events.as_mut_ptr(),
                self.events.len() as i32,
                timeout_ptr,
            )
        };

        if nevents == -1 {
            return Err(io::Error::last_os_error());
        }

        let mut ready = Vec::with_capacity(nevents as usize);

        for i in 0..nevents as usize {
            let data = self.events[i].udata as usize;
            let event = EventObject::decode(data);

            ready.push(event);
        }
        Ok(ready)
    }
}

impl Drop for OsPoller {
    fn drop(&mut self) {
        unsafe {
            close(self.kqueue_fd);
        }
    }
}
