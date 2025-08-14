use socket2::{Socket, Domain, Type, Protocol};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use clap::{Parser, value_parser, command};
use std::thread;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// IP-адрес назначения.
    #[arg(value_parser = value_parser!(Ipv4Addr))]
    ip: Ipv4Addr,
    /// Кол-во пакетов.
    #[arg(short, long, default_value = "0")]
    count: i16,
    /// Задержка (в сек.)
    #[arg(short, long)]
    duration: Option<f64>,
}

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
    let args = CliArgs::parse();

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4)).unwrap();

    let input_addr: Ipv4Addr = args.ip;
    let connect_addr = SocketAddr::new(IpAddr::from(input_addr), 8080);

    socket.connect(&connect_addr.into()).unwrap();
    println!("Подключено к {:?}", connect_addr);

    loop {
        let packet: Vec<u8> = create_packet(8, 0, 0, 1, 1, vec![0]);
        socket.send(&packet).unwrap();

        println!("Отправлен пакет! Данные {:?}", packet);
        thread::sleep(Duration::from_secs_f64(1.0))
    }
}
