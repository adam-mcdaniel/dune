use libc::{flock, LOCK_EX, LOCK_NB, LOCK_SH};
use std::io::{self, Error, ErrorKind};
use std::os::unix::io::AsRawFd;

use super::utils::syscall;
use super::{RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug)]
pub struct RwLock<T: AsRawFd> {
    pub(crate) inner: T,
}

impl<T: AsRawFd> RwLock<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        RwLock { inner }
    }

    #[inline]
    pub fn write(&mut self) -> io::Result<RwLockWriteGuard<'_, T>> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_EX) })?;
        Ok(RwLockWriteGuard::new(self))
    }

    #[inline]
    pub fn try_write(&mut self) -> Result<RwLockWriteGuard<'_, T>, Error> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_EX | LOCK_NB) }).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => ErrorKind::WouldBlock.into(),
            _ => err,
        })?;
        Ok(RwLockWriteGuard::new(self))
    }

    #[inline]
    pub fn read(&self) -> io::Result<RwLockReadGuard<'_, T>> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_SH) })?;
        Ok(RwLockReadGuard::new(self))
    }

    #[inline]
    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, T>, Error> {
        let fd = self.inner.as_raw_fd();
        syscall(unsafe { flock(fd, LOCK_SH | LOCK_NB) }).map_err(|err| match err.kind() {
            ErrorKind::AlreadyExists => ErrorKind::WouldBlock.into(),
            _ => err,
        })?;
        Ok(RwLockReadGuard::new(self))
    }

    #[inline]
    pub fn into_inner(self) -> T
    where
        T: Sized,
    {
        self.inner
    }
}
