// External modules:

use chrono::{DateTime, Utc, TimeZone};
use mysql::{OptsBuilder, Pool, from_row};


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

fn get_id_from_datetime(db_pool: &Pool, table_name: &str, station_name: &str, datetime: DateTime<Utc>) -> Result<u32> {
    // select id, battery_voltage from battery_data where timestamp = '2017-10-05 00:00:00' and station = 'Santa_Gracia';
    let query = format!("SELECT id, battery_voltage FROM {} WHERE timestamp = '{}' and station = '{}'", table_name, datetime, station_name);
    let rows = db_pool.prep_exec(query, ())?;

    let mut counter = 0;
    let mut id: u32 = 0;
    let mut battery_voltage: f64 = 0.0;

    for row in rows {
        let (id, battery_voltage): (u32, f64) = from_row(row?);
        info!("id: {}, battery_voltage: {}", id, battery_voltage);
        counter += 1;
    }

    if counter == 1 {
        Ok(id)
    } else {
        bail!("");
    }
}

fn import_simple(db_pool: Pool, station_name: &str, data: SimpleDataType) -> Result<()> {

    Ok(())
}

fn import_multiple(db_pool: Pool, station_name: &str, data: Vec<MultipleDataType>) -> Result<()> {

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
