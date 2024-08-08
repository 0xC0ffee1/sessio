use core::fmt;
use std::pin::Pin;
use std::task::Poll;
use std::task::Context;
use quinn::WriteError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::io;


pub struct BiStream {
    pub send_stream: quinn::SendStream,
    pub recv_stream: quinn::RecvStream,
}

impl tokio::io::AsyncRead for BiStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>, 
        cx: &mut std::task::Context<'_>, 
        buf: &mut tokio::io::ReadBuf<'_>
    ) -> std::task::Poll<io::Result<()>> {

        let self_mut = self.get_mut();  // Safe to call because we do not move out of the struct
        std::pin::Pin::new(&mut self_mut.recv_stream).poll_read(cx, buf)
    }
}


impl tokio::io::AsyncWrite for BiStream {
    fn poll_write(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>, 
        buf: &[u8]
    ) -> Poll<io::Result<usize>> {
        let self_mut = self.get_mut(); 
        Pin::new(&mut self_mut.send_stream)
            .poll_write(cx, buf)
            .map_err(|e| io::Error::from(e))
    }

    fn poll_flush(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<io::Result<()>> {
        let self_mut = self.get_mut();
        Pin::new(&mut self_mut.send_stream).poll_flush(cx)
    }

    fn poll_shutdown(
        self: Pin<&mut Self>, 
        cx: &mut Context<'_>
    ) -> Poll<io::Result<()>> {
        let self_mut = self.get_mut();
        Pin::new(&mut self_mut.send_stream).poll_shutdown(cx)
    }
}

impl Unpin for BiStream {}