use systemstat::{System, Platform, saturating_sub_bytes};


pub fn load_average() {
    let sys = System::new();

    match sys.load_average() {
        Ok(loadavg) => println!("\nLoad average: {} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        Err(x) => println!("\nLoad average: error: {}", x)
    }
}
