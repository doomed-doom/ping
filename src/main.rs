use std::net::{SocketAddr, TcpStream};
use std::thread::sleep;
use std::time::Duration;
use std::{io, thread};

use mio::{Events, Interest, Poll, Token, net::TcpListener};

const SERVER: Token = Token(0);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut events = Events::with_capacity(1024);
    let mut poll = Poll::new()?;

    let addr: SocketAddr = "127.0.0.1:5252".parse()?;
    let mut listener = TcpListener::bind(addr)?;

    poll.registry()
        .register(&mut listener, SERVER, Interest::READABLE)?;

    let mut threads = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let addr: SocketAddr = "127.0.0.1:5252".parse().unwrap();
            TcpStream::connect(addr).unwrap();
        });
        threads.push(handle);
    }

    threads.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which type the event is.
            match event.token() {
                SERVER => loop {
                    // One or more connections are ready, so we'll attempt to
                    // accept them (in a loop).
                    match listener.accept() {
                        Ok((_connection, address)) => {
                            println!("Got a connection from: {}", address);
                            sleep(Duration::from_secs(1));
                        }
                        // A "would block error" is returned if the operation
                        // is not ready, so we'll stop trying to accept
                        // connections.
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
