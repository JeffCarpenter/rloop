use std::num::TryFromIntError;

#[cfg(unix)]
use mio::unix::SourceFd;
#[cfg(unix)]
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

#[cfg(windows)]
use crate::io::SourceRawSocket;
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, FromRawSocket, OwnedSocket, RawSocket};

#[cfg(unix)]
pub(crate) type SocketRaw = RawFd;
#[cfg(windows)]
pub(crate) type SocketRaw = RawSocket;

#[cfg(unix)]
type OwnedSocketType = OwnedFd;
#[cfg(windows)]
type OwnedSocketType = OwnedSocket;

#[cfg(unix)]
pub(crate) type MioSocketRef<'a> = SourceFd<'a>;
#[cfg(windows)]
pub(crate) type MioSocketRef<'a> = SourceRawSocket<'a>;

pub(crate) struct SocketHandle {
    raw: SocketRaw,
    owned: OwnedSocketType,
}

#[cfg(unix)]
impl SocketHandle {
    pub(crate) unsafe fn from_raw(raw: SocketRaw) -> Self {
        Self {
            raw,
            owned: unsafe { OwnedFd::from_raw_fd(raw) },
        }
    }
}

#[cfg(windows)]
impl SocketHandle {
    pub(crate) unsafe fn from_raw(raw: SocketRaw) -> Self {
        Self {
            raw,
            owned: unsafe { OwnedSocket::from_raw_socket(raw) },
        }
    }
}

impl SocketHandle {
    pub(crate) fn try_from_i32(fd: i32) -> Result<Self, TryFromIntError> {
        #[cfg(unix)]
        {
            return Ok(unsafe { Self::from_raw(fd) });
        }

        #[cfg(windows)]
        {
            let raw = usize::try_from(fd)? as SocketRaw;
            Ok(unsafe { Self::from_raw(raw) })
        }
    }

    pub(crate) fn try_from_usize(fd: usize) -> Result<Self, TryFromIntError> {
        #[cfg(unix)]
        {
            let raw = i32::try_from(fd)?;
            return Ok(unsafe { Self::from_raw(raw) });
        }

        #[cfg(windows)]
        {
            let raw = fd as SocketRaw;
            Ok(unsafe { Self::from_raw(raw) })
        }
    }

    pub(crate) fn into_socket2(self) -> socket2::Socket {
        socket2::Socket::from(self.owned)
    }

    pub(crate) fn as_mio_source(&self) -> MioSocketRef<'_> {
        #[cfg(unix)]
        {
            SourceFd(&self.raw)
        }

        #[cfg(windows)]
        {
            SourceRawSocket(&self.raw)
        }
    }
}

#[cfg(unix)]
pub(crate) fn socket_token<S>(source: &S) -> usize
where
    S: AsRawFd,
{
    source.as_raw_fd() as usize
}

#[cfg(windows)]
pub(crate) fn socket_token<S>(source: &S) -> usize
where
    S: AsRawSocket,
{
    source.as_raw_socket() as usize
}
