extern crate clap;
extern crate serde_json;
extern crate serde;
extern crate reqwest;

use clap::{Arg, App, value_t_or_exit};
use serde::{Deserialize, Serialize};
use serde_json::{Map};
use std::collections::HashMap;

// https://blog.logrocket.com/making-http-requests-rust-reqwest/
// https://howmuchwillitsnow.com/rest/forecast/fort-collins/co
// https://transform.tools/json-to-rust-serde
// https://dev.to/pintuch/rust-serde-json-by-example-2kkf
// https://docs.rs/serde_json/0.9.10/serde_json/ <- just use serde_json:Value

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub current_temp: Option<i64>,
    pub reference_datetime: Option<String>,
    pub data_creation_datetime: Option<String>,
    pub ianaTimeZone: Option<String>,
    pub local_timezone: Option<String>,
    pub total_snow_orediction: Option<f64>,
    pub hazzards: Option<Vec<Hazzards>>,
    pub forecast_days: Option<HashMap<String, Forecast>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hazzards {
    pub name: Option<String>,
    pub url: Option<String>,
    pub start: Option<String>,
    pub end: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    pub nick: Option<String>,
    pub snow: Option<f64>,
    pub max: Option<i64>,
    pub mint: Option<i64>,
    pub cond: Option<String>,
    pub events: Option<Vec<Event>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    #[serde(rename = "type")]
    pub a_type: Option<String>,
    pub time: Option<String>,
}

fn get_parser() -> App<'static> {
     App::new("how-much-snow")
    .version("0.1.0")
    .author("Alex Balderson")
    .about("Show how much snow will fall over the next 7 days for a US City")
    .arg(
        Arg::with_name("city")
            .value_name("city")
            .takes_value(true)
    )
    .arg(
        Arg::with_name("state")
        .value_name("state")
        .takes_value(true)
    )
}

fn main() {
    let args = get_parser().get_matches();

    let city: String = value_t_or_exit!(args, "city", String);
    let state: String = value_t_or_exit!(args, "state", String);

    let url: String = format!("https://howmuchwillitsnow.com/rest/forecast/{}/{}", city, state);

    let mut response = reqwest::blocking::get(&url).expect("failed to query data");

    let json: Response = serde_json::from_str(&response.text().expect("failed to get text from data")).expect("yeah it died");

    println!("{}, {}", city, state);
    println!("{}", url);
    println!("{:?}", json);

}