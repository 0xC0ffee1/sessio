use std::pin::Pin;
use std::task::Poll;
use std::task::Context;
use russh::SshStream;
use std::io;
use log::{error, trace};  // Add the log crate for logging

pub struct BiStream {
    pub send_stream: quinn::SendStream,
    pub recv_stream: quinn::RecvStream,
}

impl SshStream for BiStream {}

impl tokio::io::AsyncRead for BiStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>, 
        cx: &mut std::task::Context<'_>, 
        buf: &mut tokio::io::ReadBuf<'_>
    ) -> std::task::Poll<io::Result<()>> {
        let self_mut = self.get_mut();
        match std::pin::Pin::new(&mut self_mut.recv_stream).poll_read(cx, buf) {
            Poll::Ready(Ok(())) => {
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => {
                error!("Error reading from recv_stream: {}", e);
                Poll::Ready(Err(e))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl tokio::io::AsyncWrite for BiStream {
    fn poll_write(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>, 
        buf: &[u8]
    ) -> Poll<io::Result<usize>> {
        let self_mut = self.get_mut(); 
        match Pin::new(&mut self_mut.send_stream).poll_write(cx, buf) {
            Poll::Ready(Ok(size)) => {
                Poll::Ready(Ok(size))
            }
            Poll::Ready(Err(e)) => {
                error!("Error writing to send_stream: {}", e);
                Poll::Ready(Err(io::Error::from(e)))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<io::Result<()>> {
        let self_mut = self.get_mut();
        match Pin::new(&mut self_mut.send_stream).poll_flush(cx) {
            Poll::Ready(Ok(())) => {
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => {
                error!("Error flushing send_stream: {}", e);
                Poll::Ready(Err(e))
            }
            Poll::Pending => Poll::Pending,
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<io::Result<()>> {
        let self_mut = self.get_mut();
        match Pin::new(&mut self_mut.send_stream).poll_shutdown(cx) {
            Poll::Ready(Ok(())) => {
                trace!("Shutdown of send_stream succeeded");
                Poll::Ready(Ok(()))
            }
            Poll::Ready(Err(e)) => {
                error!("Error shutting down send_stream: {}", e);
                Poll::Ready(Err(e))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Unpin for BiStream {}
