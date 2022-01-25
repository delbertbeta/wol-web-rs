use eui48::{MacAddress, ParseError};
use std::net::{Ipv4Addr, UdpSocket};

const REPEAT_TIME: usize = 16;
const OFFSET: usize = 6;
const MAC_LENGTH: usize = 6;

#[derive(Debug)]
pub struct MagicPacket {
    magic_bytes: [u8; 102],
    port: u16,
}

impl MagicPacket {
    pub fn new(mac_address: &str, port: u16) -> Result<MagicPacket, ParseError> {
        let mac = MacAddress::parse_str(mac_address)?;
        let mac_address = mac.to_array();

        let mut magic_bytes = [0xFF; 102];

        for repeat in 0..REPEAT_TIME {
            for offset in 0..MAC_LENGTH {
                let index = repeat * MAC_LENGTH + OFFSET + offset;
                magic_bytes[index] = mac_address[offset];
            }
        }

        Ok(MagicPacket { magic_bytes, port })
    }

    pub fn send(&self) -> std::io::Result<()> {
        let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0))?;
        socket.set_broadcast(true)?;
        socket.send_to(
            &self.magic_bytes,
            (Ipv4Addr::new(255, 255, 255, 255), self.port),
        )?;

        Ok(())
    }
}
