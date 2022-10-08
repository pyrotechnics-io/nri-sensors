use log;
use std::{thread, time};
use simple_logger::SimpleLogger;
mod arguments;
mod sensors;
mod statsgen;

#[macro_use]
extern crate error_chain;

mod errors {
    error_chain!{}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = match arguments::process_arguments() {
        Ok(a) => a,
        Err(e) => {
            bail!("Error processing command line: {}", e);
        }
    };

    SimpleLogger::new()
        .with_level(opts.debugmode)
        .init()
        .unwrap();
    log::info!("New Relic LM Sensors");

    log::debug!("Polling at {} second intervals", &opts.frequency);

    loop {
        let millis = time::Duration::from_secs(opts.frequency);
        let data = sensors::poll();
        log::debug!("Discovered {} temperature metrics", data.len());

        statsgen::dump(data);

        thread::sleep(millis);
    }
}
