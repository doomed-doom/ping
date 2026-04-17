use clap::{Parser, value_parser};
use std::{net::Ipv4Addr, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CliArgs {
    /// IP-адрес назначения.
    #[arg(value_parser = value_parser!(Ipv4Addr))]
    ip: Ipv4Addr,
    /// Кол-во пакетов.
    #[arg(short, long, default_value = "0", value_parser = value_parser!(usize))]
    count: usize,
    /// Задержка (в сек.)
    #[arg(short, long, default_value = "1")]
    duration: f64,
}

impl CliArgs {
    pub fn parse_args() -> Self {
        CliArgs::parse()
    }
    
    pub fn ip(&self) -> Ipv4Addr {
        self.ip
    }
    
    pub fn count(&self) -> usize {
        self.count
    }
    
    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.duration)
    }

    pub fn get_all_args(&self) -> (Ipv4Addr, usize, Duration) {
        (self.ip(), self.count(), self.duration())
    }
}
