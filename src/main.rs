extern crate systemstat;

mod track_metrics;

use std::thread;
use std::time::Duration;
use clokwerk::{Scheduler, TimeUnits};

fn main() {
    let mut scheduler = Scheduler::with_tz(chrono::Utc);

    scheduler.every(1.minutes()).run(|| track_metrics::load_average());

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
     }

}