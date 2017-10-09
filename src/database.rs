// External modules:

use chrono::{NaiveDateTime};
use mysql::{OptsBuilder, Pool, from_row, Value};

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

fn get_id_from_datetime(db_pool: &Pool, table_name: &str, station_name: &str, datetime: NaiveDateTime) -> Result<Option<u32>> {
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
            let query = format!("UPDATE battery_data SET timestamp = :timestamp, station = :station, battery_voltage = :battery_voltage, li_battery_voltage = :li_battery_voltage, wind_dir = :wind_dir WHERE id = '{}'", id);

            info!("query: '{}'", query);

            db_pool.prep_exec(query, (
                Value::from(data.date_time),
                Value::from(station_name),
                Value::from(data.solar_battery_voltage),
                Value::from(data.lithium_battery_voltage),
                Value::from(data.wind_direction)
            ))?;

            info!("single with id");
        },
        None => {
            let query = format!("INSERT INTO battery_data (timestamp, station, battery_voltage, li_battery_voltage, wind_dir) VALUES (:timestamp, :station, :battery_voltage, :li_battery_voltage, :wind_dir)");

            info!("query: '{}'", query);

            db_pool.prep_exec(query, (
                Value::from(data.date_time),
                Value::from(station_name),
                Value::from(data.solar_battery_voltage),
                Value::from(data.lithium_battery_voltage),
                Value::from(data.wind_direction)
            ))?;

            info!("single without id");
        }
    }

    Ok(())
}

fn import_multiple(db_pool: Pool, station_name: &str, data: Vec<MultipleDataType>) -> Result<()> {
    for data in data {
        match get_id_from_datetime(&db_pool, "multiple_data", station_name, data.date_time)? {
            Some(id) => {
                let query = format!("UPDATE multiple_data SET timestamp = :timestamp, station = :station, air_temperature = :air_temperature, air_relative_humidity = :air_relative_humidity, solar_radiation = :solar_radiation,
                    soil_water_content = :soil_water_content, soil_temperature = :soil_temperature, wind_speed = :wind_speed, wind_max = :wind_max, wind_direction = :wind_direction, precipitation = :precipitation,
                    air_pressure = :air_pressure WHERE id = '{}'", id);

                info!("query: '{}'", query);

                db_pool.prep_exec(query, (
                    Value::from(data.date_time),
                    Value::from(station_name),
                    Value::from(data.air_temperature),
                    Value::from(data.air_relative_humidity),
                    Value::from(data.solar_radiation),
                    Value::from(data.soil_water_content),
                    Value::from(data.soil_temperature),
                    Value::from(data.wind_speed),
                    Value::from(data.wind_max),
                    Value::from(data.wind_direction),
                    Value::from(data.precipitation),
                    Value::from(data.air_pressure)
                ))?;

                info!("multiple with id");
            },
            None => {
                let query = format!("INSERT INTO multiple_data (timestamp, station, air_temperature, air_relative_humidity, solar_radiation, soil_water_content, soil_temperature,
                    wind_speed, wind_max, wind_direction, precipitation, air_pressure) VALUES (:timestamp, :station, :air_temperature, :air_relative_humidity,
                    :solar_radiation, :soil_water_content, :soil_temperature, :wind_speed, :wind_max, :wind_direction, :precipitation, :air_pressure)");

                info!("query: '{}'", query);

                db_pool.prep_exec(query, (
                    Value::from(data.date_time),
                    Value::from(station_name),
                    Value::from(data.air_temperature),
                    Value::from(data.air_relative_humidity),
                    Value::from(data.solar_radiation),
                    Value::from(data.soil_water_content),
                    Value::from(data.soil_temperature),
                    Value::from(data.wind_speed),
                    Value::from(data.wind_max),
                    Value::from(data.wind_direction),
                    Value::from(data.precipitation),
                    Value::from(data.air_pressure)
                ))?;

                info!("multiple without id");
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
