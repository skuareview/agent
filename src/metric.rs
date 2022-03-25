// use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

#[derive(Default, Clone)]
pub struct Metric {
    loads: (f32, f32, f32),
    memory: (String, String),
    cpu_temp: f32,
    cpu_load: f32,
}

impl Metric {
    // Main method
    pub fn get_metrics(&mut self) {
        let sys = System::new();
        println!("\n");
        match sys.load_average() {
            Ok(loadavg) => self.set_loads(loadavg.one, loadavg.five, loadavg.fifteen),
            Err(x) => println!("Load average: error: {}", x),
        }

        match sys.memory() {
            Ok(mem) => self.set_memory(
                saturating_sub_bytes(mem.total, mem.free).to_string(),
                mem.total.to_string(),
            ),
            Err(x) => println!("Memory: error: {}", x),
        }

        match sys.cpu_temp() {
            Ok(cpu_temp) => self.set_cpu_temp(cpu_temp),
            Err(x) => println!("CPU temp: {}", x),
        }

        match sys.cpu_load_aggregate() {
            Ok(cpu) => {
                println!("Measuring CPU load...");
                thread::sleep(Duration::from_secs(1));
                let cpu = cpu.done().unwrap();
                self.set_cpu_load(cpu.system);
            }
            Err(x) => println!("CPU load: error: {}", x),
        }
    }

    // Setters
    fn set_loads(&mut self, one: f32, five: f32, fifteen: f32) {
        self.loads = (one, five, fifteen);
        println!(
            "Load average {} {} {}",
            self.loads.0, self.loads.1, self.loads.2
        );
    }
    fn set_memory(&mut self, used: String, total: String) {
        self.memory = (used, total);
        println!("Memory {} / {}", self.memory.0, self.memory.1);
    }
    fn set_cpu_temp(&mut self, cpu_temp: f32) {
        self.cpu_temp = cpu_temp;
        println!("CPU temp {}", self.cpu_temp);
    }
    fn set_cpu_load(&mut self, cpu_load: f32) {
        self.cpu_load = cpu_load;
        println!("CPU load {}%", self.cpu_load);
    }
}
