use clap::{App, Arg};
use log::LevelFilter;

pub struct Arguments {
    pub frequency: u64,
    pub debugmode: LevelFilter,
}

pub fn process_arguments() -> Result<Arguments, &'static str> {
    let matches = App::new("New Relic LM Sensors")
        .version(env!("BUILD_TAG"))
        .author("Harsh Baste <hbaste@newrelic.com>")
        .about(
            "Monitors lm-sensor data and publishes it to New Relic via the infrastructure agent",
        )
        .arg(
            Arg::with_name("frequency")
                .short('f')
                .long("frequency")
                .takes_value(true)
                .help("Poll/publish frequency in seconds"),
        )
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .action(clap::ArgAction::Count)
                .help("Debug verbosity"),
        )
        .get_matches();

    let debugmode = match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        _ => return Err("Only 2 levels of debug permitted"),
    };

    let frequency = matches.value_of("frequency");
    let frequency = match frequency {
        None => 120,
        Some(s) => match s.parse::<u64>() {
            Ok(n) => n,
            Err(_) => return Err("Invalid frequency number"),
        },
    };

    Ok(Arguments{
        frequency,
        debugmode,
    })
}