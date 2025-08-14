use socket2::{Socket, Domain, Type, Protocol, SockAddr};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

struct IcmpPacket {
    icmp_type: u8,
    code: u8,
    checksum: u16,
    id: u16,
    seq_num: u16,
    payload: Vec<u8>,
}

impl IcmpPacket {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.payload.len());

        bytes.push(self.icmp_type);
        bytes.push(self.code);
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.seq_num.to_be_bytes());

        bytes.extend_from_slice(&self.payload);

        bytes
    }
}

fn create_packet(packet_type: u8, packet_code: u8, packet_checksum: u16, packet_id: u16, packet_seq_num: u16, payload: Vec<u8>) -> Vec<u8>{
    let packet = IcmpPacket {
        icmp_type: packet_type,
        code: packet_code,
        checksum: packet_checksum,
        id: packet_id,
        seq_num: packet_seq_num,
        payload,
    };

    packet.to_bytes()
}

fn main() {
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4)).unwrap();
    let connect_addr: SockAddr = SockAddr::from(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000));

    socket.connect(&connect_addr).unwrap();
    println!("Подключено к {:?}", connect_addr);

    let packet: Vec<u8> = create_packet(8, 0, 0, 1, 1, vec![0]);
    socket.send(&packet).unwrap();
    println!("Отправлен пакет! Данные {:?}", packet)
}
