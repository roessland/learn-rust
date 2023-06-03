//#![allow(dead_code, unused_variables)]

mod monitor {
    #[derive(Debug)]
    pub struct Resolution {
        pub width: u16,
        pub height: u16,
    }

    #[derive(Debug)]
    pub struct Monitor {
        tech: String,
        resolution: Resolution,
        has_stand: bool,
        serial_number: u32,
        big_data: Vec<u8>,
    }

    pub type OptionFn = Box<dyn FnOnce(&mut Monitor) -> ()>;

    pub fn new(options: Vec<OptionFn>) -> Monitor {
        let mut monitor = Monitor {
            resolution: Resolution {
                width: 0,
                height: 0,
            },
            has_stand: false,
            tech: String::from(""),
            serial_number: 0,
            big_data: vec![],
        };
        for option_fn in options {
            option_fn(&mut monitor);
        }
        monitor
    }

    pub fn with_stand() -> OptionFn {
        Box::new(|monitor: &mut Monitor| {
            monitor.has_stand = true;
        })
    }

    pub fn with_resolution(w: u16, h: u16) -> OptionFn {
        Box::new(move |monitor: &mut Monitor| {
            monitor.resolution = Resolution { width: w, height: h };
        })
    }

    pub fn with_tech(tech: &str) -> OptionFn {
        let tech = tech.to_string();
        Box::new(move |monitor: &mut Monitor| {
            monitor.tech = tech.clone();
        })
    }

    use std::sync::atomic::{AtomicU32, Ordering};

    static SERIAL_NUMBER_COUNTER: AtomicU32 = AtomicU32::new(1);

    pub fn with_serial_number() -> OptionFn {
        Box::new(|monitor: &mut Monitor| {
            monitor.serial_number = SERIAL_NUMBER_COUNTER.fetch_add(1, Ordering::SeqCst);
        })
    }

    pub fn with_big_data(big_data: Vec<u8>) -> OptionFn {
        Box::new(|monitor: &mut Monitor| {
            monitor.big_data = big_data
        })
    }
}

fn main() {
    let big_data = vec![0; 1024 * 1024];
    let mon = monitor::new(vec![
        monitor::with_stand(),
        monitor::with_resolution(1920, 1080),
        monitor::with_tech("OLED"),
        monitor::with_serial_number(),
        monitor::with_big_data(big_data),
    ]);

    println!("{:?}", mon);
}
