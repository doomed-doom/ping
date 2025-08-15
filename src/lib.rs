use std::time::{Duration, Instant};
use std::net::Ipv4Addr;

#[allow(dead_code)]
pub struct PingStats {
    host: Ipv4Addr,
    count: u16,
    transmitted: u16,
    received: u16,
    loss: u16,
    start: Instant,
    end: Instant,
    total_delay: Duration,
}