use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

#[allow(dead_code)]
pub struct PingStats {
    host: Ipv4Addr,
    transmitted: usize,
    received: usize,
    loss: usize,
    start: Instant,
    end: Instant,
    avg_delay: f64,
}

impl PingStats {
    pub fn new(host: Ipv4Addr,
               transmitted: usize,
               received: usize,
               loss: usize,
               start: Instant,
               end: Instant,
               delays: Vec<Duration>) -> Self {
        let avg_delay: f64 = {
            let mut all_delay: f64 = 0.0;
            for dur in &delays {
                all_delay += dur.as_secs_f64();
            };
            all_delay / delays.len() as f64
        };

        PingStats {
            host,
            transmitted,
            received,
            loss,
            start,
            end,
            avg_delay,
        }
    }
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
