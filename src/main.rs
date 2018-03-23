// External crates:
#[macro_use] extern crate log;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate nom;
#[macro_use] extern crate combine;

extern crate simplelog;
extern crate time;
extern crate regex;
extern crate chrono;
extern crate byteorder;
extern crate clap;
extern crate mysql;

// Internal modules:
mod error;
mod data_parser;
mod database;

// External modules:
use clap::{App, Arg};
use chrono::Local;
use simplelog::{Config, TermLogger, WriteLogger, LogLevelFilter};
use log::LogLevel;

// System modules:
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Read;

// Internal modules:
use error::{Result, ResultExt};
use data_parser::{parse_data};
use database::{import_to_db};

quick_main!(|| -> Result<()> {

    let matches = App::new("sbd_db_import")
        .version("0.2")
        .author("Willi Kappler")
        .about("Import binary SBD files from weatherstations into the database, files sent via e-mail")
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
            .help("The binary SBD file")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let db_user = matches.value_of("db_user").unwrap();
    let db_password = matches.value_of("db_password").unwrap();
    let station_name = matches.value_of("station").unwrap();
    let file_name = matches.value_of("file_name").unwrap();

    // Initialize logger
    let dt = Local::now();
    let log_filename = dt.format("sbd_db_import_%Y_%m_%d.log").to_string();

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

    if file_name.starts_with("300025060000500") {
        info!("Pan_de_Azucar");
    } else if file_name.starts_with("300025060004660") {
        info!("La_Campana");
    } else if file_name.starts_with("300025060007390") {
        info!("Santa_Gracia");
    } else if file_name.starts_with("300025060008580") {
        info!("Nahuelbuta");
    }

    let mut input_file = File::open(file_name).chain_err(|| format!("Could not open sbd file: '{}'", file_name))?;
    let mut binary_data = Vec::new();
    let data_size = input_file.read_to_end(&mut binary_data)?;

    info!("Bytes read: {}", data_size);

    let weatherstation_data = parse_data(binary_data)?;

    info!("data: {:?}", weatherstation_data);

    import_to_db(db_user, db_password, station_name, weatherstation_data)?;

    info!("import successfull to database");

    Ok(())
});
