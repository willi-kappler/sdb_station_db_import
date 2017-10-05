// External crates:
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate mysql;

extern crate simplelog;
extern crate time;
extern crate regex;
extern crate chrono;
extern crate byteorder;

// Internal modules:
mod error;

// External crates:
use clap::{App, Arg};
use chrono::Local;
use std::fs::OpenOptions;
use simplelog::{Config, TermLogger, WriteLogger, LogLevelFilter};
use log::LogLevel;
use mysql::{OptsBuilder, Pool};

// Internal modules:
use error::{Result};

quick_main!(|| -> Result<()> {

    let matches = App::new("iridium_weatherstation")
        .version("0.1")
        .author("Willi Kappler")
        .about("Import binary SDB files from weatherstations into the database, files sent via e-mail")
        .arg(
            Arg::with_name("db_name")
            .long("db_name")
            .help("Name of the database")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("db_user")
            .long("db_user")
            .help("Username for the database")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("db_password")
            .long("db_password")
            .help("Password for the database")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("station")
            .long("station")
            .help("The name of the weatherstation")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("file")
            .long("file")
            .help("The binary SDB file")
            .takes_value(true)
        )
        .get_matches();

    let db_name = matches.value_of("db_name").ok_or("db name missing")?;
    let db_user = matches.value_of("db_user").ok_or("db user missing")?;
    let db_password = matches.value_of("db_password").ok_or("db password missing")?;
    let station = matches.value_of("station").ok_or("station missing")?;
    let file = matches.value_of("file").ok_or("file missing")?;

    // Initialize logger
    let dt = Local::now();
    let log_filename = dt.format("sdb_db_import_%Y_%m_%d.log").to_string();

    let log_config = Config {
        time: Some(LogLevel::Warn),
        level: Some(LogLevel::Warn),
        target: Some(LogLevel::Warn),
        location: Some(LogLevel::Warn)
    };

    if let Ok(file) = OpenOptions::new().append(true).create(true).open(&log_filename) {
        let _ = WriteLogger::init(LogLevelFilter::Info, log_config, file);
        info!("Log file '{}' created succesfully", &log_filename);
    } else {
        // Log file could not be created, use stdout instead
        let _ = TermLogger::init(LogLevelFilter::Info, log_config);
        warn!("Could not open log fle: '{}', using sdtout instead!", &log_filename);
    }

    let mut db_builder = OptsBuilder::new();
    db_builder.ip_or_hostname(Some("localhost"))
           .db_name(Some(db_name))
           .user(Some(db_user))
           .pass(Some(db_password));

    let db_pool = Pool::new(db_builder)?;

    info!("Connected to database '{}'", db_name);










    Ok(())
});
