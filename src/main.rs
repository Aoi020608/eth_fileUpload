use byteorder::{LittleEndian, BigEndian};

fn main() {
    println!("Hello, world!");
}

struct Version {
   version: LittleEndian,
    services: LittleEndian,
    timestamp: LittleEndian,
    addr_recv: NetAddr,
    addr_from: NetAddr,
    nonce: LittleEndian,
    user_agent: LittleEndian,
    start_height: LittleEndian,
    relay: bool,
}

struct MessageHeader {
    magic: LittleEndian,
    command_name: String,
    payload_size: LittleEndian,
    check_sum: String,
} 

struct NetAddr {
    services: LittleEndian,
    ip: String,
    port: BigEndian,
}

struct Message {
    command_name: String,
}

impl Message {
    
}

