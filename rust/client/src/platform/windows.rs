
use tokio::io::{self, AsyncRead, AsyncWrite, ReadBuf};
use futures_util::stream::Stream;

use std::{io::{Read, Write}, pin::Pin};
use std::task::{Context, Poll};

use tonic::transport::server::Connected;
use uds_windows::{UnixListener, UnixStream};

pub struct UnixListenerStream {
    pub inner: UnixListener,
}

impl UnixListenerStream {
    pub fn new(inner: UnixListener) -> Self {
        Self { inner }
    }
}

impl Stream for UnixListenerStream {
    type Item = Result<UnixStreamWrapper, io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.accept() {
            Ok((stream, _)) => Poll::Ready(Some(Ok(UnixStreamWrapper(stream)))),
            Err(e) => {
                if e.kind() == tokio::io::ErrorKind::WouldBlock {
                    cx.waker().wake_by_ref();
                    Poll::Pending
                } else {
                    Poll::Ready(Some(Err(e)))
                }
            }
        }
    }
}

pub struct UnixStreamWrapper(UnixStream);

impl AsyncRead for UnixStreamWrapper {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let mut unfilled = buf.initialize_unfilled();
        match Pin::new(&mut self.0).read(&mut unfilled) {
            Ok(n) => {
                buf.advance(n);
                Poll::Ready(Ok(()))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl AsyncWrite for UnixStreamWrapper {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match Pin::new(&mut self.0).write(buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        match Pin::new(&mut self.0).flush() {
            Ok(_) => Poll::Ready(Ok(())),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<()>> {
        match Pin::new(&mut self.0).shutdown(std::net::Shutdown::Write) {
            Ok(_) => Poll::Ready(Ok(())),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}

impl Connected for UnixStreamWrapper {
    type ConnectInfo = ();

    fn connect_info(&self) -> Self::ConnectInfo {
        ()
    }
}
