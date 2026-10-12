use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use mio::{Events, Interest, Poll, Token, net::TcpListener};

use ping::IcmpAsyncSocket;

const ICMP_SOCKET: Token = Token(0);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(unix))]
    panic!("The unix-like systems only supported");

    let addr: SocketAddr = "127.0.0.1:0".parse()?;

    let mut events = Events::with_capacity(1024);
    let mut poll = Poll::new()?;

    let mut icmp_sock = IcmpAsyncSocket::new()?;

    poll.registry().register(
        &mut icmp_sock,
        ICMP_SOCKET,
        Interest::WRITABLE | Interest::READABLE,
    )?;

    loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        for event in events.iter() {
            match event.token() {
                ICMP_SOCKET => loop {
                    match icmp_sock.connect(addr) {
                        Ok(()) => {
                            println!("Successfully connected to {addr}");
                        }
                        Err(ref err) if would_block(err) => break,
                        Err(err) => return Err(Box::new(err)),
                    }
                },
                _ => println!("Wrong token!"),
            }
        }
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}
