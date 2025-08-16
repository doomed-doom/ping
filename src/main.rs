use socket2::{Socket, Domain, Type, Protocol};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use clap::{Parser, value_parser, command};
use std::thread;
use std::time::Duration;
use std::mem::MaybeUninit;
use ping::PingStats;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// IP-адрес назначения.
    #[arg(value_parser = value_parser!(Ipv4Addr))]
    ip: Ipv4Addr,
    /// Кол-во пакетов.
    #[arg(short, long, default_value = "0", value_parser = value_parser!(usize))]
    count: usize,
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
    let input_addr: Ipv4Addr = args.ip;
    let count: usize = args.count;
    let dur: Duration = Duration::from_secs_f64(args.duration.unwrap());

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4)).unwrap();
    let connect_addr = SocketAddr::new(IpAddr::from(input_addr), 0);

    socket.connect(&connect_addr.into()).unwrap();

    let mut buf: [MaybeUninit<u8>; 1500] = unsafe { MaybeUninit::uninit().assume_init() };

    let mut send = 0;
    let mut recv = 0;

    let packet_len: usize = create_packet(8, 0, 0, 1, 1, vec![0]).len();

    println!("Начинаем пинг {} - {} байт данных.",
        input_addr, packet_len
    );

    while count == 0 || send < count {
        let packet: Vec<u8> = create_packet(8, 0, 0, 1, 1, vec![0]);
        socket.send(&packet).unwrap();
        send += 1;

        let len = socket.recv(&mut buf).unwrap();
        let bytes: &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, len) };

        if len > 0 && (bytes[1] == 0u8) {
            println!("{} байт от {}",
                len, input_addr
            );
            recv += 1;
        }

        if send != count {
            thread::sleep(dur);
        }
    }
    println!("--- Статистика пинга {} ---", input_addr);
    println!("Отправлено: {send} пакетов.\nПолучено: {recv}. Потери: {}%", (send - recv));
}
