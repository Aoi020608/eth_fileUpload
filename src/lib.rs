use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    io::{self, Read, Write},
    net::Ipv4Addr,
    sync::{mpsc, Arc, Mutex},
    thread,
};

mod tcp;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

type InterfaceHandle = Arc<Mutex<ConnectionManager>>;

enum InterfaceRequest {
    Write {
        quad: Quad,
        bytes: Vec<u8>,
        ack: mpsc::Sender<usize>,
    },
    Flush {
        quad: Quad,
        ack: mpsc::Sender<()>,
    },
    Bind {
        port: u16,
        ack: mpsc::Sender<()>,
    },
    Unbind,
    Read {
        quad: Quad,
        max_length: usize,
        read: mpsc::Sender<Vec<u8>>,
    },
    Accept {
        port: u16,
        ack: mpsc::Sender<Quad>,
    },
}

pub struct Interface {
    ih: InterfaceHandle,
    jh: thread::JoinHandle<()>,
}

#[derive(Default)]
struct ConnectionManager {
    connections: HashMap<Quad, tcp::Connection>,
    pending: HashMap<u16, VecDeque<Quad>>,
}

impl Interface {
    pub fn new() -> io::Result<Self> {
        let nic = tun_tap::Iface::without_packet_info("tun0", tun_tap::Mode::Tun)?;

        let cm: InterfaceHandle = Arc::default();
        let jh = {
            let cm = cm.clone();
            thread::spawn(move || {
                let nic = nic;
                let cm = cm;
                let buf = [0u8; 1504];

                // do the stuff that main does
            })
        };

        Ok(Interface { ih: cm, jh })
    }

    pub fn bind(&mut self, port: u16) -> io::Result<TcpListener> {
        let mut cm = self.ih.lock().unwrap();
        match cm.pending.entry(port) {
            Entry::Vacant(v) => {
                v.insert(VecDeque::new());
            }
            Entry::Occupied(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::AddrInUse,
                    "port already bound",
                ));
            }
        }
        drop(cm);
        Ok(TcpListener(port, self.ih.clone()))
    }
}

pub struct TcpListener(u16, InterfaceHandle);

impl TcpListener {
    pub fn try_accept(&mut self) -> io::Result<TcpStream> {
        let mut cm = self.1.lock().unwrap();
        if let Some(quad) = cm
            .pending
            .get_mut(&self.0)
            .expect("port closed while listener still active")
            .pop_front()
        {
            Ok(TcpStream(quad, self.1.clone()))
        } else {
            // TODO: block
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "no connection to accept",
            ));
        }
    }
}

pub struct TcpStream(Quad, InterfaceHandle);

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut cm = self.1.lock().unwrap();
        let c = cm.connections.get_mut(&self.0).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "tcp was terminated unexpectedly.",
            )
        })?;

        if c.incoming.is_empty() {
            // TODO: block
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "no bytes to read",
            ));
        }

        // TODO: detect FIN and return nread == 0

        // read as much as we ca from the incoming buffer
        let mut nread = 0;
        let (head, tail) = c.incoming.as_slices();
        let hread = std::cmp::min(buf.len(), head.len());
        buf.copy_from_slice(&head[..hread]);
        nread += hread;

        let tread = std::cmp::min(buf.len() - nread, tail.len());
        buf.copy_from_slice(&tail[..tread]);
        nread += tread;
        drop(c.incoming.drain(..nread));
        
        Ok(nread)
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut cm = self.1.lock().unwrap();
        let c = cm.connections.get_mut(&self.0).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::ConnectionAborted,
                "tcp was terminated unexpectedly.",
            )
        })?;

        if c.incoming.is_empty() {
            // TODO: block
            return Err(io::Error::new(
                io::ErrorKind::WouldBlock,
                "no bytes to read",
            ));
        }

        // TODO: detect FIN and return nread == 0

        // read as much as we ca from the incoming buffer
        let mut nread = 0;
        let (head, tail) = c.incoming.as_slices();
        let hread = std::cmp::min(buf.len(), head.len());
        buf.copy_from_slice(&head[..hread]);
        nread += hread;

        let tread = std::cmp::min(buf.len() - nread, tail.len());
        buf.copy_from_slice(&tail[..tread]);
        nread += tread;
        drop(c.incoming.drain(..nread));
        
        Ok(nread)
    }

    fn flush(&mut self) -> io::Result<()> {
        // let (ack, rx) = mpsc::channel();
        // self.1.send(InterfaceRequest::Flush { quad: self.0, ack });
        // rx.recv().unwrap();
        // Ok(())
        Ok(())
    }
}
