# pollio

A small native event poller abstraction over Linux epoll and macOS kqueue.

## Example

```rust
use pollio::{Event, Interest, OsPoller, Poller};

fn main() -> std::io::Result<()> {
    let mut poller = OsPoller::new()?;

    // poller.add(fd, token, Interest::Readable)?;
    // let events = poller.wait(-1)?;

    Ok(())
}