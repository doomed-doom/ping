use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

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
