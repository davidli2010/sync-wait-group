//! Enables threads to synchronize the beginning or end of some computation.
//!
//! # Examples
//!
//! ```
//! use sync_wait_group::WaitGroup;
//! use std::thread;
//!
//! // Create a new wait group.
//! let wg = WaitGroup::new();
//!
//! for _ in 0..4 {
//!     // Create another reference to the wait group.
//!     let wg = wg.clone();
//!
//!     thread::spawn(move || {
//!         // Do some work.
//!
//!         // Drop the reference to the wait group.
//!         drop(wg);
//!     });
//! }
//!
//! // Block until all threads have finished their work.
//! wg.wait();
//! ```

use parking_lot::{Condvar, Mutex};
use std::fmt;
use std::sync::Arc;

/// Enables threads to synchronize the beginning or end of some computation.
pub struct WaitGroup {
    inner: Arc<Inner>,
}

/// Inner state of a `WaitGroup`.
struct Inner {
    cvar: Condvar,
    count: Mutex<usize>,
}

impl Default for WaitGroup {
    #[inline]
    fn default() -> Self {
        WaitGroup::new()
    }
}

impl WaitGroup {
    /// Creates a new wait group and returns the single reference to it.
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                cvar: Condvar::new(),
                count: Mutex::new(1),
            }),
        }
    }

    /// Drops this reference and waits until all other references are dropped.
    #[inline]
    pub fn wait(self) {
        if *self.inner.count.lock() == 1 {
            return;
        }

        let inner = self.inner.clone();
        drop(self);

        let mut count = inner.count.lock();
        while *count > 0 {
            inner.cvar.wait(&mut count);
        }
    }
}

impl Drop for WaitGroup {
    #[inline]
    fn drop(&mut self) {
        let mut count = self.inner.count.lock();
        *count -= 1;

        if *count == 0 {
            self.inner.cvar.notify_all();
        }
    }
}

impl Clone for WaitGroup {
    #[inline]
    fn clone(&self) -> WaitGroup {
        let mut count = self.inner.count.lock();
        *count += 1;

        WaitGroup {
            inner: self.inner.clone(),
        }
    }
}

impl fmt::Debug for WaitGroup {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let count: &usize = &*self.inner.count.lock();
        f.debug_struct("WaitGroup").field("count", count).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    const THREADS: usize = 10;

    #[test]
    fn wait() {
        let wg = WaitGroup::new();
        let (tx, rx) = std::sync::mpsc::channel();

        for _ in 0..THREADS {
            let wg = wg.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                wg.wait();
                tx.send(()).unwrap();
            });
        }

        thread::sleep(Duration::from_millis(100));

        // At this point, all spawned threads should be blocked, so we shouldn't get anything from the
        // channel.
        assert!(rx.try_recv().is_err());

        wg.wait();

        // Now, the wait group is cleared and we should receive messages.
        for _ in 0..THREADS {
            rx.recv().unwrap();
        }
    }

    #[test]
    fn wait_and_drop() {
        let wg = WaitGroup::new();
        let (tx, rx) = std::sync::mpsc::channel();

        for _ in 0..THREADS {
            let wg = wg.clone();
            let tx = tx.clone();

            thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));
                tx.send(()).unwrap();
                drop(wg);
            });
        }

        // At this point, all spawned threads should be sleeping, so we shouldn't get anything from the
        // channel.
        assert!(rx.try_recv().is_err());

        wg.wait();

        // Now, the wait group is cleared and we should receive messages.
        for _ in 0..THREADS {
            rx.try_recv().unwrap();
        }
    }
}
