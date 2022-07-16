use std::io::Result;
use std::net::{SocketAddr, UdpSocket};

fn create_magic_packet(mac_addr: &[u8; 6]) -> [u8; 102] {
    let mut magic_packet: Vec<u8> = vec![];

    // 0xFF * 6 times
    magic_packet.extend(&[0xFFu8].repeat(6));

    // mac_addr (6 bytes) * 16 times = 96
    magic_packet.extend(&mac_addr.repeat(16));

    // try to convert to [u8; 102] and return or return [0; 102]
    magic_packet.try_into().unwrap_or([0_u8; 102])
}

fn parse_input(addr: String) -> [u8; 6] {
    let bytes: Vec<u8> = addr
        .split(":")
        .map(|b| u8::from_str_radix(b, 16).unwrap_or(0x00))
        .collect();

    bytes.try_into().unwrap_or([0_u8; 6])
}

fn main() -> Result<()> {
    let arg = std::env::args().nth(1);

    if arg.is_none() {
        println!(
            "{}",
            "Error: Target MAC address expected. Use: wol-rs MAC ADDRESS"
        );
        return Ok(());
    }

    let arg = arg.unwrap();

    let from: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 0));
    let broadcast: SocketAddr = SocketAddr::from(([255, 255, 255, 255], 9));

    let socket = UdpSocket::bind(from)?;
    socket.set_broadcast(true)?;

    socket.send_to(&create_magic_packet(&parse_input(arg)), broadcast)?;

    Ok(())
}
