use mio::{Interest, Registry, Token, event::Source as MioSource, net::TcpListener};
use std::io;

#[cfg(windows)]
use std::os::windows::io::RawSocket;
#[cfg(unix)]
use {mio::unix::SourceFd, std::convert::TryFrom, std::os::fd::RawFd};

#[derive(Clone, Copy, Debug)]
pub(crate) struct OsSocket {
    #[cfg(unix)]
    raw: RawFd,
    #[cfg(windows)]
    raw: RawSocket,
}

impl OsSocket {
    #[cfg(unix)]
    pub(crate) fn new(id: usize) -> Self {
        let raw = i32::try_from(id).expect("file descriptor exceeds RawFd range");
        Self { raw }
    }

    #[cfg(windows)]
    pub(crate) fn new(id: usize) -> Self {
        Self { raw: id as RawSocket }
    }

    #[cfg(unix)]
    fn register(self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        let mut fd = SourceFd(&self.raw);
        registry.register(&mut fd, token, interests)
    }

    #[cfg(windows)]
    fn register(self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        registry.register_raw_socket(self.raw, token, interests)
    }

    #[cfg(unix)]
    fn reregister(self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        let mut fd = SourceFd(&self.raw);
        registry.reregister(&mut fd, token, interests)
    }

    #[cfg(windows)]
    fn reregister(self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        registry.reregister_raw_socket(self.raw, token, interests)
    }

    #[cfg(unix)]
    fn deregister(self, registry: &Registry) -> io::Result<()> {
        let mut fd = SourceFd(&self.raw);
        registry.deregister(&mut fd)
    }

    #[cfg(windows)]
    fn deregister(self, registry: &Registry) -> io::Result<()> {
        registry.deregister_raw_socket(self.raw)
    }
}

pub(crate) enum Source {
    Socket(OsSocket),
    TCPListener(TcpListener),
}

impl Source {
    #[inline]
    pub(crate) fn socket(id: usize) -> Self {
        Self::Socket(OsSocket::new(id))
    }
}

impl MioSource for Source {
    #[inline]
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        match self {
            Self::Socket(socket) => (*socket).register(registry, token, interests),
            Self::TCPListener(listener) => listener.register(registry, token, interests),
        }
    }

    #[inline]
    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest) -> io::Result<()> {
        match self {
            Self::Socket(socket) => (*socket).reregister(registry, token, interests),
            Self::TCPListener(listener) => listener.reregister(registry, token, interests),
        }
    }

    #[inline]
    fn deregister(&mut self, registry: &Registry) -> io::Result<()> {
        match self {
            Self::Socket(socket) => (*socket).deregister(registry),
            Self::TCPListener(listener) => listener.deregister(registry),
        }
    }
}
