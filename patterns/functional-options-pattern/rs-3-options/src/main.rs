#![allow(dead_code, unused_variables)]
#![feature(trait_upcasting)]
#![allow(incomplete_features)]

mod monitor {
    #[derive(Debug)]
    pub struct Resolution {
        pub width: u16,
        pub height: u16,
    }

    #[derive(Debug)]
    pub struct Monitor {
        pub tech: String,
        pub resolution: Resolution,
        pub has_stand: bool,
    }

    pub type MonitorModifier = Box<dyn FnOnce(&mut Monitor)>;

    pub fn new(mut modifiers: Vec<MonitorModifier>) -> Monitor {
        let mut monitor = Monitor {
            resolution: Resolution {
                width: 0,
                height: 0,
            },
            has_stand: false,
            tech: String::from("OLED"),
        };

        for modifier in modifiers.drain(..) {
            modifier(&mut monitor);
        }
        monitor
    }

    pub fn with_stand() -> Box<dyn FnMut(&mut Monitor)> {
        Box::new(|monitor: &mut Monitor| {
            monitor.has_stand = true;
        })
    }

    pub fn with_tech(tech: String) -> Box<dyn FnMut(&mut Monitor)> {
        Box::new(move |monitor: &mut Monitor| {
            monitor.tech = tech.clone();
        })
    }

    pub fn with_resolution(w: u16, h: u16) -> Box<dyn FnMut(&mut Monitor)> {
        Box::new(move |monitor: &mut Monitor| {
            monitor.resolution = Resolution { width: w, height: h };
        })
    }
}

fn main() {
    let mon;
    {
        mon = monitor::new(vec![
            monitor::with_stand(),
            monitor::with_tech("OLED".into()),
            monitor::with_resolution(1920, 1080),
        ]);
    }

    println!("{:?}", mon)
}
