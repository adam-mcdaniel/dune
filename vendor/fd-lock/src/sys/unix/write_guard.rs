use libc::{flock, LOCK_UN};
use std::ops;
use std::os::unix::io::AsRawFd;

use super::utils::syscall;
use super::RwLock;

#[derive(Debug)]
pub struct RwLockWriteGuard<'lock, T: AsRawFd> {
    lock: &'lock mut RwLock<T>,
}

impl<'lock, T: AsRawFd> RwLockWriteGuard<'lock, T> {
    pub(crate) fn new(lock: &'lock mut RwLock<T>) -> Self {
        Self { lock }
    }
}

impl<T: AsRawFd> ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.lock.inner
    }
}

impl<T: AsRawFd> ops::DerefMut for RwLockWriteGuard<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lock.inner
    }
}

impl<T: AsRawFd> Drop for RwLockWriteGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        let fd = self.lock.inner.as_raw_fd();
        let _ = syscall(unsafe { flock(fd, LOCK_UN) });
    }
}
