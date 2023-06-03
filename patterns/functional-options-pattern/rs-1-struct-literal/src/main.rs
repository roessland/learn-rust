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
}

fn main() {
    let mon = monitor::Monitor {
        tech: String::from("OLED"),
        resolution: monitor::Resolution {
            width: 1920,
            height: 1080,
        },
        has_stand: true,
    };
    println!("{:?}", mon)
}
