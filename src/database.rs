// External modules:

use chrono::{DateTime, Utc, TimeZone};
use mysql;


// Internal modules:
use error::{Result};
use data_parser::WeatherStationData;

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


pub fn import_to_db(db_user: &str, db_password: &str, station_name: &str, data: WeatherStationData) -> Result<()> {

    Ok(())
}
