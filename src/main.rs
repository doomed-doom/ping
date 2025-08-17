use clap::{Parser, command, value_parser};
use socket2::{Domain, Protocol, Socket, Type};
use std::mem::MaybeUninit;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::{process, thread};
use std::time::{Duration, Instant};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering}
};

use ping::PingStats;

mod consts;
use consts::{ICMP_ECHO_ANSWER_TYPE, ICMP_ECHO_REQUEST_TYPE};

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
    #[arg(short, long, default_value = "1")]
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

fn create_packet(
    packet_type: u8,
    packet_code: u8,
    packet_checksum: u16,
    packet_id: u16,
    packet_seq_num: u16,
    payload: Vec<u8>,
) -> Vec<u8> {
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

fn connect(ip_addr: Ipv4Addr, socket: &Socket) -> std::io::Result<()> {
    let connect_addr = SocketAddr::new(IpAddr::from(ip_addr), 0);

    socket.connect(&connect_addr.into())?;
    Ok(())
}
fn main() {
    let args = CliArgs::parse();

    let ip_addr: Ipv4Addr = args.ip;
    let count: usize = args.count;
    let dur: Duration = Duration::from_secs_f64(args.duration.unwrap());

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4)).unwrap();
    connect(ip_addr.clone(), &socket).unwrap();

    let mut buf: [MaybeUninit<u8>; 1500] = unsafe { MaybeUninit::uninit().assume_init() };

    let (mut send, mut recv) = (0, 0);

    let packet_len: usize = create_packet(ICMP_ECHO_REQUEST_TYPE, 0, 0, 1, 1, vec![0]).len();

    println!(
        "Начинаем пинг {} - {} байт данных пакет.",
        ip_addr, packet_len
    );

    let mut ping_delays: Vec<Duration> = Vec::new();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        process::exit(0);
    }).unwrap();

    let start = Instant::now();
    while count == 0 || send < count {
        if send > 0 {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(dur);
        }

        let packet = create_packet(ICMP_ECHO_REQUEST_TYPE, 0, 0, 1, send as u16, vec![0]);
        let start_send = Instant::now();
        socket.send(&packet).unwrap();
        send += 1;

        let len = socket.recv(&mut buf).unwrap();
        let bytes: &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, len) };

        let end_send = Instant::now();
        if len > 0 && (bytes[1] == ICMP_ECHO_ANSWER_TYPE) {
            println!("{} байт от {}", len, ip_addr);
            recv += 1;
        }

        ping_delays.push(end_send - start_send);
    }
    let end = Instant::now();

    let result = PingStats::new(
        ip_addr,
        send,
        recv,
        send - recv,
        start,
        end,
        ping_delays,
    );

    println!("{result}");
}
