// External crates:
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate mysql;
#[macro_use] extern crate nom;

extern crate simplelog;
extern crate time;
extern crate regex;
extern crate chrono;
extern crate byteorder;

// Internal modules:
mod error;
mod data_parser;

// External modules:
use clap::{App, Arg};
use chrono::Local;
use simplelog::{Config, TermLogger, WriteLogger, LogLevelFilter};
use log::LogLevel;
use mysql::{OptsBuilder, Pool};

// System modules:
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;

// Internal modules:
use error::{Result, ResultExt};
use data_parser::{parse_data};

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
            .required(true)
        )
        .arg(
            Arg::with_name("db_user")
            .long("db_user")
            .help("Username for the database")
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("db_password")
            .long("db_password")
            .help("Password for the database")
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("station")
            .long("station")
            .help("The name of the weatherstation")
            .takes_value(true)
            .required(true)
        )
        .arg(
            Arg::with_name("file_name")
            .long("file_name")
            .help("The binary SDB file")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let db_name = matches.value_of("db_name").unwrap();
    let db_user = matches.value_of("db_user").unwrap();
    let db_password = matches.value_of("db_password").unwrap();
    let station = matches.value_of("station").unwrap();
    let file_name = matches.value_of("file_name").unwrap();

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


    let mut input_file = File::open(file_name).chain_err(|| format!("Could not open file: '{}'", file_name))?;
    let mut binary_data = Vec::new();
    let data_size = input_file.read_to_end(&mut binary_data)?;

    info!("Bytes read: {}", data_size);

    let weatherstation_data = parse_data(binary_data)?;

    info!("data: {:?}", weatherstation_data);

    let mut db_builder = OptsBuilder::new();
    db_builder.ip_or_hostname(Some("localhost"))
           .db_name(Some(db_name))
           .user(Some(db_user))
           .pass(Some(db_password));

    let db_pool = Pool::new(db_builder)?;

    info!("Connected to database '{}'", db_name);










    Ok(())
});
