extern crate systemstat;

mod config;
mod metric;

use clokwerk::{Scheduler, TimeUnits};
use config::Config;
use metric::Metric;
use std::thread;
use std::time::Duration;

fn main() {
    let mut config = Config::default();
    config.get_config();
    println!("token : {}", config.get_token());

    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    let mut metric = Metric::default();

    // Each minute run the get_metrics method
    scheduler
        .every(1.minutes())
        .run(move || metric.get_metrics());

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
