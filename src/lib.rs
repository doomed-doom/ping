use std::fmt::{Display, Formatter};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

use socket2::Socket;

pub struct IcmpPacket {
    icmp_type: u8,
    code: u8,
    checksum: u16,
    id: u16,
    seq_num: u16,
    payload: Vec<u8>,
}

impl IcmpPacket {
    pub fn new(
        packet_type: u8,
        packet_code: u8,
        packet_checksum: u16,
        packet_id: u16,
        packet_seq_num: u16,
        payload: Vec<u8>,
    ) -> Self {
        IcmpPacket {
            icmp_type: packet_type,
            code: packet_code,
            checksum: packet_checksum,
            id: packet_id,
            seq_num: packet_seq_num,
            payload,
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(8 + self.payload.len());

        bytes.push(self.icmp_type);
        bytes.push(self.code);
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.seq_num.to_be_bytes());
        bytes.extend_from_slice(&self.payload);

        bytes
    }

    pub fn packet_len(&self) -> usize {
        self.to_bytes().len()
    }
}

#[derive(Clone)]
pub struct PingStats {
    host: Ipv4Addr,
    start: Instant,
    transmitted: Option<usize>,
    received: Option<usize>,
    end: Option<Instant>,
    avg_delay: Option<f64>,
}

impl PingStats {
    pub fn new(host: Ipv4Addr, start: Instant) -> Self {
        PingStats {
            host,
            start,
            transmitted: None,
            received: None,
            end: None,
            avg_delay: None,
        }
    }

    pub fn finish(
        &mut self,
        end: Instant,
        transmitted: &usize,
        received: &usize,
        delays: &Vec<Duration>,
    ) -> () {
        let avg_delay: f64 = if delays.is_empty() {
            0.0
        } else {
            delays.iter().map(Duration::as_secs_f64).sum::<f64>() / delays.len() as f64
        };

        self.end = Some(end);
        self.transmitted = Some(*transmitted);
        self.received = Some(*received);
        self.avg_delay = Some(avg_delay);
    }
}

impl Display for PingStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n--- Статистика пинга {} ---\nОтправлено: {} пакетов.\nПолучено: {}, потери: {}%, общее время: {:.4}\nСредняя задержка (rtt): {:.4}",
            self.host,
            self.transmitted.unwrap(),
            self.received.unwrap(),
            self.transmitted.unwrap() - self.received.unwrap(),
            (self.end.unwrap() - self.start).as_secs_f64(),
            self.avg_delay.unwrap(),
        )
    }
}

pub fn connect(ip_addr: Ipv4Addr, socket: &Socket) -> std::io::Result<()> {
    let connect_addr = SocketAddr::new(IpAddr::from(ip_addr), 0);

    socket.connect(&connect_addr.into())?;
    Ok(())
}

pub mod consts;
pub mod cli;
