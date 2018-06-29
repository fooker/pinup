extern crate clap;
extern crate ctrlc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;
extern crate sysfs_gpio;

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use sysfs_gpio::{Direction, Edge, Error, Pin};

mod config;


static RUNNING: AtomicBool = AtomicBool::new(true);


fn handle(pin: u64, config: config::Pin) -> Result<(), Error> {
    let pin = Pin::new(pin);

    pin.with_exported(|| {
        pin.set_direction(Direction::In)?;
        pin.set_edge(Edge::BothEdges)?;

        let mut last = (pin.get_value()? != 0, Instant::now());

        let mut poller = pin.get_poller()?;
        while RUNNING.load(Ordering::Relaxed) {
            if let Some(raw) = poller.poll(100)? {
                let curr = (config.inverted ^ (raw != 0), Instant::now());

                if curr.0 != last.0 && curr.1 - last.1 >= Duration::from_millis(config.debounce) {
                    if curr.0 {
                        println!("Triggered '{}'", config.name);

                        Command::new("sh")
                                .arg("-c")
                                .arg(config.script.to_owned())
                                .spawn()
                                .expect("Failed to spawn script");
                    }

                    last = curr;
                } else {
                    last.0 = curr.0;
                }
            }
        }

        return Ok(());
    })?;

    return Ok(());
}


fn main() {
    let args = clap::App::new("pinup")
            .version("0.1")
            .author("Dustin Frisch <fooker@lab.sh>")
            .about("Triggers scripts on GPIO events")
            .arg(clap::Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .default_value("config.yaml")
                    .help("Specify the config file to use")
                    .takes_value(true))
            .get_matches();

    // Load config
    let config = config::Config::load(args.value_of("config").unwrap()).expect("Failed to read config");

    // Handle each configured pin
    let mut threads = vec![];
    for (pin, config) in config.pins {
        threads.push(thread::spawn(move || handle(pin, config)));
    }

    ctrlc::set_handler(move || {
        RUNNING.store(false, Ordering::Relaxed);
    }).expect("Error setting Ctrl-C handler");

    for thread in threads {
        thread.join()
              .expect("Error waiting for child")
              .expect("Error in child");
    }
}
