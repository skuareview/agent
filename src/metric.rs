use systemstat::{System, Platform};

#[derive(Default,Copy, Clone)]
pub struct Metric {
    loads: (f32,f32,f32)
}

impl Metric {

    // Main method
    pub fn get_metrics(self) {
        let sys = System::new();
        match sys.load_average() {
            Ok(loadavg) => self.set_loads(loadavg.one, loadavg.five, loadavg.fifteen),
            Err(x) => println!("\nLoad average: error: {}", x)
        }
    }
    
    // Setters
    fn set_loads(mut self, one:f32,five:f32,fifteen:f32) {
        self.loads = (one,five,fifteen);
        println!("\nLoad average {} {} {}",self.loads.0,self.loads.1,self.loads.2);
    }
    
}