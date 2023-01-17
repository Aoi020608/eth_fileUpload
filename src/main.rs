use std::{borrow::Cow, net::{SocketAddr, IpAddr, Ipv4Addr}};

use bitcoin::{network::{constants::ServiceFlags, Address}};

fn main() {
    let s4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(173, 242, 112, 53)), 8333);
    let a4 = Address::new(&s4, ServiceFlags::NETWORK | ServiceFlags::WITNESS);
    println!("Hello");
}

// trait Version {
//     fn is_verack(&self) -> bool;
//     fn is_version(&self) -> Option<VersionMessage>;
// }
// 
// enum NetworkMessage {
//     Version(VersionMessage),
//     Verack,
// }

// struct VersionMessage {
//    version: LittleEndian,
//     services: LittleEndian,
//     timestamp: LittleEndian,
//     addr_recv: Address,
//     addr_from: Address,
//     nonce: LittleEndian,
//     user_agent: LittleEndian,
//     start_height: LittleEndian,
//     relay: bool,
// }

struct MessageHeader {
    magic: u32,
    command: CommandString,
    length: u32,
    checksum: u32,
    payload: Vec<u8>,
} 

struct CommandString(Cow<'static, str>);

struct Message {
    command_name: String,
}

impl Message {
    
}

