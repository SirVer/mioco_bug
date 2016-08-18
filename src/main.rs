#![recursion_limit = "4096"]
#[macro_use] extern crate mioco;

use mioco::tcp::{TcpListener, TcpStream};
use std::borrow::Cow;
use std::io::{self, Read, Write};
use std::net::SocketAddr;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::{Arc, RwLock};

fn main() {
    mioco::start(|| -> io::Result<()> {
        let addr: SocketAddr = FromStr::from_str("0.0.0.0:50000").unwrap();
        let listener = try!(TcpListener::bind(&addr));
        loop {
            let mut conn = try!(listener.accept());
            mioco::spawn(move || -> io::Result<()> {
                let (tx, rx) = mioco::sync::mpsc::sync_channel(5);
                
                let mut buf = vec![0u8; 10];
                loop {
                    select!(
                        r:conn => {
                            while let Some(len) = try!(conn.try_read(&mut buf)) {
                                tx.send(10).unwrap();
                            }
                        },
                        r:rx => {
                            while let Ok(data) = rx.try_recv() {}
                        }
                    );
                }
                Ok(())
            });
        }
    }).unwrap().unwrap();
}
