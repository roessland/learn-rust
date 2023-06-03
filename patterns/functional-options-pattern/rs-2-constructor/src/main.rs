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

    pub fn new(tech: String, resolution: Resolution, has_stand: bool) -> Result<Monitor, String> {
        if tech == "" {
            return Err(String::from("tech field is empty"));
        }
        let mon = Monitor {
            tech,
            resolution,
            has_stand,
        };
        if let Err(e) = validate(&mon) {
            return Err(e);
        }
        Ok(mon)
    }

    fn validate(monitor: &Monitor) -> Result<(), String> {
        if monitor.tech == "" {
            return Err(String::from("tech field is empty"));
        } // ... more validations here
        Ok(())
    }
}

fn main() {
    let mon1 = monitor::new(
        String::from("OLED"),
        monitor::Resolution { width: 1920, height: 1080 },
        true,
    );

    println!("{:?}", mon1);

    // Still allowed to create instances directly.
    // let mon2 = monitor::Monitor { ... };
}
