use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;
use std::time::Instant;

#[allow(dead_code)]
pub struct PingStats {
    pub host: Ipv4Addr,
    pub transmitted: usize,
    pub received: usize,
    pub loss: usize,
    pub start: Instant,
    pub end: Instant,
    pub avg_delay: f64,
}

impl Display for PingStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\n--- Статистика пинга {} ---\nОтправлено: {} пакетов.\nПолучено: {}, потери: {}%, общее время: {:.4}\nСредняя задержка (rtt): {:.4}",
            self.host,
            self.transmitted,
            self.received,
            self.loss,
            (self.end - self.start).as_secs_f64(),
            self.avg_delay
        )
    }
}
