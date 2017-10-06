// External modules:
use diesel;

// Internal modules:
use error::{Result};
use data_parser::WeatherStationData;



pub fn import_to_db(db_name: &str, db_user: &str, db_password: &str, station_name: &str, data: WeatherStationData) -> Result<()> {



    Ok(())
}
