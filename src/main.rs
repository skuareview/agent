extern crate systemstat;

mod metric;

use std::thread;
use std::time::Duration;
use clokwerk::{Scheduler, TimeUnits};
use metric::Metric;

fn main() {
    let mut scheduler = Scheduler::with_tz(chrono::Utc);
    let metric = Metric::default();

    // Each minute run the get_metrics method
    scheduler.every(1.minutes()).run(move || metric.get_metrics());

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }

}