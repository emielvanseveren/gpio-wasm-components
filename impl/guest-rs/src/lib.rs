use crate::thesis::gpio::types::{gpio_init, Mode};
use std::env;
use std::time::{Duration, Instant};

wit_bindgen::generate!({
    path: "../wit",
    world: "gpio-app",
});

struct Component;

impl GpioApp for Component {
    fn start() -> Result<(), ()> {
        let start = Instant::now();
        gpio_init(0, 17, Mode::Output).unwrap();
        println!(
            "instance={},action=guest-to-host,elapsed={:?}",
            env::var("INSTANCE_NUMBER").unwrap(),
            start.elapsed().as_micros()
        );

        std::thread::sleep(Duration::from_secs(10));
        Ok(())
    }
    fn host_to_guest() -> Result<(), ()> {
        Ok(())
    }
}

export_gpio_app!(Component);
