use ping::{IcmpPacket, connect};
use socket2::{Domain, Protocol, Socket, Type};
use std::mem::MaybeUninit;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};
use std::time::{Duration, Instant};
use std::{process, thread};

use ping::{
    PingStats,
    consts::{ICMP_ECHO_ANSWER_TYPE, ICMP_ECHO_REQUEST_TYPE},
};

fn main() {
    let cli_args = ping::cli::CliArgs::parse_args();
    let (ip_addr, count, dur) = cli_args.get_all_args();

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::ICMPV4)).unwrap();
    connect(ip_addr.clone(), &socket).unwrap();

    let mut buf: [MaybeUninit<u8>; 1500] = unsafe { MaybeUninit::uninit().assume_init() };

    let (sent, recv) = (Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0)));

    let packet_len: usize =
        IcmpPacket::new(ICMP_ECHO_REQUEST_TYPE, 0, 0, 1, 1, vec![0]).packet_len();

    println!(
        "Начинаем пинг {} - {} байт данных пакет.",
        ip_addr, packet_len
    );

    let ping_delays: Arc<Mutex<Vec<Duration>>> = Arc::new(Mutex::new(Vec::new()));

    let running = Arc::new(AtomicBool::new(true));
    let r = Arc::clone(&running);

    let stats = Arc::new(Mutex::new(PingStats::new(ip_addr, Instant::now())));

    {
        let sent = Arc::clone(&sent);
        let recv = Arc::clone(&recv);
        let ping_delays = Arc::clone(&ping_delays);
        let stats = Arc::clone(&stats);

        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);

            let sent_val = sent.load(Ordering::SeqCst);
            let recv_val = recv.load(Ordering::SeqCst);
            let delays = ping_delays.lock().unwrap().clone();

            let mut stats_copy = stats.lock().unwrap().clone();
            stats_copy.finish(Instant::now(), &sent_val, &recv_val, &delays);

            println!("{}", stats_copy);

            process::exit(0);
        })
        .unwrap();
    }

    while count == 0 || sent.load(Ordering::SeqCst) < count {
        if sent.load(Ordering::SeqCst) > 0 {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            thread::sleep(dur);
        }

        let packet = IcmpPacket::new(
            ICMP_ECHO_REQUEST_TYPE,
            0,
            0,
            1,
            sent.load(Ordering::SeqCst) as u16,
            vec![0],
        );

        let start_send = Instant::now();
        socket.send(&packet.to_bytes()).unwrap();
        sent.fetch_add(1, Ordering::SeqCst);

        let len = socket.recv(&mut buf).unwrap();
        let bytes: &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, len) };

        let end_send = Instant::now();
        if len > 0 && bytes[1] == ICMP_ECHO_ANSWER_TYPE {
            println!(
                "{} байт от {}: icmp_seq={} time={} ms",
                len,
                ip_addr,
                sent.load(Ordering::SeqCst),
                (end_send.clone() - start_send.clone()).as_millis()
            );
            recv.fetch_add(1, Ordering::SeqCst);
        }

        ping_delays.lock().unwrap().push(end_send - start_send);
    }

    let (sent_val, recv_val) = (sent.load(Ordering::SeqCst), recv.load(Ordering::SeqCst));
    let delays = ping_delays.lock().unwrap().clone();

    let mut stats_copy = stats.lock().unwrap().clone();
    stats_copy.finish(Instant::now(), &sent_val, &recv_val, &delays);

    println!("{}", stats_copy);
}
