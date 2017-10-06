
// External modules:
use nom::{le_u32, be_u16, IResult};
use chrono::{DateTime, Utc, TimeZone};

// System modules:
use std::f64::{INFINITY, NEG_INFINITY, NAN};


// Internal modules:
use error::{Result, ErrorKind};

#[derive(Debug)]
pub struct SimpleDataType {
    pub date_time: DateTime<Utc>,
    pub solar_battery_voltage: f64,
    pub lithium_battery_voltage: f64,
    pub wind_direction: f64,
}

#[derive(Debug)]
pub struct MultipleDataType {
    pub date_time: DateTime<Utc>,
    pub air_temperature: f64,
    pub air_relative_humidity: f64,
    pub solar_radiation: f64,
    pub soil_water_content: f64,
    pub soil_temperature: f64,
    pub wind_speed: f64,
    pub wind_max: f64,
    pub wind_direction: f64,
    pub precipitation: f64,
    pub air_pressure: f64,
}

#[derive(Debug)]
pub enum WeatherStationData {
    SimpleData(SimpleDataType),
    MultipleData(Vec<MultipleDataType>),
}

fn u16_to_f64(data: u16) -> f64 {
    // base16 2 byte floats:
    // https://en.wikipedia.org/wiki/Half-precision_floating-point_format
    // https://github.com/sgothel/jogl/blob/master/src/jogl/classes/com/jogamp/opengl/math/Binary16.java
    // https://books.google.de/books?id=FPlICAAAQBAJ&pg=PA84&lpg=PA84&dq=binary16&source=bl&ots=0FAzD4XOqn&sig=98h_pzPlLzUXjB4uY1T8MRIZOnA&hl=de&sa=X&ved=0ahUKEwjkpvXU5ZzLAhVD9HIKHQOfAxYQ6AEITzAH#v=onepage&q=binary16&f=false
    // http://www.gamedev.net/topic/557338-ieee-754-2008-binary-16-inaccuracy-in-wikipedia/

    // Campbells own 2 bytes floating point format:
    // Bits: ABCDEFGH IJKLMNOP
    //
    // A: Sign, 0: +, 1: -
    //
    // B, C: Decimal position (exponent):
    // 0, 0: XXXX.
    // 0, 1: XXX.X
    // 1, 0: XX.XX
    // 1, 1: X.XXX
    //
    // D: being the MSB
    //
    // E-P: 13-bit binary value, Largest 13-bit magnitude (mantissa) is 8191, but Campbell Scientific defines the largest-allowable magnitude as 7999
    //
    // More information here:
    // https://www.campbellsci.com/forum?forum=1&l=thread&tid=540

    // 17660 = 252 + (68 * 256) = 01000100 11111100 -> 12.76
    // 17662 = 254 + (68 * 256) = 01000100 11111110 -> 12.78
    // 17664 = 69 * 256 =  01000101 00000000 -> 12.80
    // 24576 = (96 * 256) = 01100000 00000000 -> 0
    // 962 = 194 + (3 * 256) = 00000011 11000011 -> 963.0
    // 25576 = 232 + (99 * 256) = 01100011 11101000 -> 1.0

    const F2_POS_INFINITY: u16 = 0b00011111_11111111; // 31, 255
    const F2_NEG_INFINITY: u16 = 0b10011111_11111111; // 159, 255
    const F2_NAN: u16 = 0b10011111_11111110; // 159, 254

    if data == F2_POS_INFINITY {
        INFINITY
    } else if data == F2_NEG_INFINITY {
        NEG_INFINITY
    } else if data == F2_NAN {
        NAN
    } else {
        let sign = if data & 0b10000000_00000000 == 0 { 1.0 } else { - 1.0 };

        let mantissa: f64 = ((data & 0b00011111_11111111) as f64) * sign;
        let exponent: u16 = (data & 0b01100000_00000000) >> 13;

        match exponent {
            1 => mantissa / 10.0,
            2 => mantissa / 100.0,
            3 => mantissa / 1000.0,
            _ => mantissa
        }
    }
}




named!(parse_date_time<&[u8], DateTime<Utc> >, do_parse!(
    seconds: le_u32 >>
    le_u32 >> // unused, since all zero
    (Utc.timestamp((seconds + 631152000) as i64, 0))
    // date_time: 2017-09-13T13:00:00Z, 631152000
    // date_time: 2017-09-13T12:00:00Z, 631148400
));

named!(parse_data_simple<&[u8], WeatherStationData>, do_parse!(
    date_time: parse_date_time >>
    solar_battery_voltage: be_u16 >> // solar battery voltage
    lithium_battery_voltage: be_u16 >> // lithium battery valotage
    wind_direction: be_u16 >> // wind diagnose
    (
        WeatherStationData::SimpleData(SimpleDataType {
            date_time: date_time,
            solar_battery_voltage: u16_to_f64(solar_battery_voltage),
            lithium_battery_voltage: u16_to_f64(lithium_battery_voltage),
            wind_direction: u16_to_f64(wind_direction),
        })
    )
));

named!(parse_data_multiple_one<&[u8], MultipleDataType>, do_parse!(
    date_time: parse_date_time >>
    air_temperature: be_u16 >>
    air_relative_humidity: be_u16 >>
    solar_radiation: be_u16 >>
    soil_water_content: be_u16 >>
    soil_temperature: be_u16 >>
    wind_speed: be_u16 >>
    wind_max: be_u16 >>
    wind_direction: be_u16 >>
    precipitation: be_u16 >>
    air_pressure: be_u16 >>
    (
        MultipleDataType {
            date_time: date_time,
            air_temperature: u16_to_f64(air_temperature),
            air_relative_humidity: u16_to_f64(air_relative_humidity),
            solar_radiation: u16_to_f64(solar_radiation),
            soil_water_content: u16_to_f64(soil_water_content),
            soil_temperature: u16_to_f64(soil_temperature),
            wind_speed: u16_to_f64(wind_speed),
            wind_max: u16_to_f64(wind_max),
            wind_direction: u16_to_f64(wind_direction),
            precipitation: u16_to_f64(precipitation),
            air_pressure: u16_to_f64(air_pressure),
        }
    )
));

named!(parse_data_multiple<&[u8], WeatherStationData>, do_parse!(
    multiple: many1!(parse_data_multiple_one) >>
    (
        WeatherStationData::MultipleData(multiple)
    )
));

named!(multiple_or_simple<&[u8], WeatherStationData>, do_parse!(
    result: alt!(complete!(parse_data_multiple) | complete!(parse_data_simple)) >>
    ( result )
));

pub fn parse_data(binary_data: Vec<u8>) -> Result<WeatherStationData> {
    match multiple_or_simple(&binary_data) {
        IResult::Done(rest, result) => {
            Ok(result)
        },
        IResult::Error(err) => {
            Err(ErrorKind::ParseError(format!("error: {:?}", err)).into())
        },
        IResult::Incomplete(needed) => {
            Err(ErrorKind::ParseError(format!("more input needed: {:?}", needed)).into())
        }
    }
}
