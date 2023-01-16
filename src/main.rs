use std::borrow::Cow;

use byteorder::{LittleEndian, BigEndian};

fn main() {
    println!("Hello, world!");
}

trait Version {
    fn is_verack(&self) -> bool;
    fn is_version(&self) -> Option<VersionMessage>;
}

enum NetworkMessage {
    Version(VersionMessage),
    Verack,
}

struct VersionMessage {
   version: LittleEndian,
    services: LittleEndian,
    timestamp: LittleEndian,
    addr_recv: Address,
    addr_from: Address,
    nonce: LittleEndian,
    user_agent: LittleEndian,
    start_height: LittleEndian,
    relay: bool,
}

struct MessageHeader {
    magic: u32,
    command: CommandString,
    length: u32,
    checksum: u32,
    payload: Vec<u8>,
} 

struct Address {
    services: ServiceFlags,
    address: [u16; 8],
    port: u16,
}

struct ServiceFlags(u64);

struct CommandString(Cow<'static, str>);

struct Message {
    command_name: String,
}

impl Message {
    
}

