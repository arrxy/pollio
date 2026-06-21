use crate::EventObject;
use std::io;
use std::os::fd::RawFd;

pub trait Poller {
    fn new() -> io::Result<Self>
    where
        Self: Sized;

    fn add(&self, event: EventObject) -> io::Result<()>;

    fn delete(&self, fd: RawFd) -> io::Result<()>;

    fn wait(&mut self, timeout_ms: i32) -> io::Result<Vec<EventObject>>;
}