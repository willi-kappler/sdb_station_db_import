// External modules:

use chrono::{DateTime, Utc, TimeZone};
use mysql;
use mysql::{OptsBuilder, Pool, from_row, Row};

use std;

// Internal modules:
use error::{Result};
use data_parser::{WeatherStationData, SimpleDataType, MultipleDataType};

/*
| id                 | int(10) unsigned | NO   | PRI | NULL    | auto_increment |
| timestamp          | datetime         | YES  | MUL | NULL    |                |
| station            | varchar(32)      | YES  |     | NULL    |                |
| battery_voltage    | double           | YES  |     | NULL    |                |
| li_battery_voltage | double           | YES  |     | NULL    |                |
| wind_dir           | double           | YES  |     | NULL    |                |
*/

/*
| id                    | int(10) unsigned | NO   | PRI | NULL    | auto_increment |
| timestamp             | datetime         | YES  | MUL | NULL    |                |
| station               | varchar(32)      | YES  |     | NULL    |                |
| air_temperature       | double           | YES  |     | NULL    |                |
| air_relative_humidity | double           | YES  |     | NULL    |                |
| solar_radiation       | double           | YES  |     | NULL    |                |
| soil_water_content    | double           | YES  |     | NULL    |                |
| soil_temperature      | double           | YES  |     | NULL    |                |
| wind_speed            | double           | YES  |     | NULL    |                |
| wind_max              | double           | YES  |     | NULL    |                |
| wind_direction        | double           | YES  |     | NULL    |                |
| precipitation         | double           | YES  |     | NULL    |                |
| air_pressure          | double           | YES  |     | NULL    |                |
*/

fn get_id_from_datetime(db_pool: &Pool, table_name: &str, station_name: &str, datetime: DateTime<Utc>) -> Result<Option<u32>> {
    // select id, battery_voltage from battery_data where timestamp = '2017-10-05 00:00:00' and station = 'Santa_Gracia';
    let query = format!("SELECT id FROM {} WHERE timestamp = '{}' and station = '{}'", table_name, datetime, station_name);
    // let rows = db_pool.prep_exec(query, ())?.map(|row: std::result::Result<Row, mysql::Error>| from_row(row?));

    let mut result: Vec<(u32,)> = Vec::new();

    for row in db_pool.prep_exec(query, ())? {
        result.push(from_row(row?));
    }

    if result.len() == 0 {
        info!("no entry found with datetime: {}", datetime);
        Ok(None)
    } else if result.len() == 1 {
        let (id,) = result[0];
        info!("id from database: {}, datetime: {}", id, datetime);
        Ok(Some(id))
    } else {
        bail!("expected exactly one id from database but got: {:?}", result);
    }
}

fn import_simple(db_pool: Pool, station_name: &str, data: SimpleDataType) -> Result<()> {
    match get_id_from_datetime(&db_pool, "battery_data", station_name, data.date_time)? {
        Some(id) => {

        },
        None => {

        }
    }

    Ok(())
}

fn import_multiple(db_pool: Pool, station_name: &str, data: Vec<MultipleDataType>) -> Result<()> {
    for data in data {
        match get_id_from_datetime(&db_pool, "multiple_data", station_name, data.date_time)? {
            Some(id) => {

            },
            None => {
                
            }
        }
    }

    Ok(())
}

pub fn import_to_db(db_user: &str, db_password: &str, station_name: &str, data: WeatherStationData) -> Result<()> {
    let mut db_builder = OptsBuilder::new();
    db_builder.ip_or_hostname(Some("localhost"))
        .db_name(Some("weatherstation"))
        .user(Some(db_user))
        .pass(Some(db_password));

    let db_pool = Pool::new(db_builder)?;

    info!("Connected to database");

    match data {
        WeatherStationData::SimpleData(data) => {
            import_simple(db_pool, station_name, data)
        },
        WeatherStationData::MultipleData(data) => {
            import_multiple(db_pool, station_name, data)
        }
    }
}
