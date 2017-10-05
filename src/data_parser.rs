
// External modules:
use nom::{le_u32, le_u16, IResult};
use chrono::{DateTime, Utc, TimeZone};

// Internal modules:
use error::{Result, ErrorKind};

#[derive(Debug)]
struct SimpleDataType {
    date_time: DateTime<Utc>,
}

#[derive(Debug)]
struct MultipleDataType {
    date_time: DateTime<Utc>,
}

#[derive(Debug)]
pub enum WeatherStationData {
    SimpleData(SimpleDataType),
    MultipleData(Vec<MultipleDataType>),
}

named!(parse_date_time<&[u8], DateTime<Utc> >, do_parse!(
    seconds: le_u32 >>
    le_u32 >> // unused, since all zero
    (Utc.timestamp(seconds as i64, 0))
));

named!(parse_data_simple<&[u8], WeatherStationData>, do_parse!(
    date_time: parse_date_time >>
    le_u16 >> // solar battery voltage
    le_u16 >> // lithium battery valotage
    le_u16 >> // wind diagnose
    (
        WeatherStationData::SimpleData(SimpleDataType {
            date_time: date_time,
        })
    )
));

named!(parse_data_multiple_one<&[u8], MultipleDataType>, do_parse!(
    date_time: parse_date_time >>
    (
        MultipleDataType {
            date_time: date_time,
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
